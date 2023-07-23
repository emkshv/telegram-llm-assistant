use crate::config::Config;
use crate::db::chat_bot;
use crate::db::chat_message;
use crate::db::chat_thread;
use crate::llm;
use crate::llm::llm_thread_message;
use anyhow::{anyhow, Context, Result};
use mobot::*;
use sqlx::{Pool, Sqlite};
use std::sync::Arc;
use std::{collections::HashMap, env};
use tokio::sync::RwLock;

#[derive(Clone)]
enum UserChatState {
    WaitingBehaviorInput,
    Default,
}

#[derive(Clone, Default, BotState)]
struct RunningBotState {
    db_pool: Option<Pool<Sqlite>>,
    user_chat_state: Arc<RwLock<HashMap<i64, UserChatState>>>,
    config: Config,
}

async fn handle_get_version(
    _e: Event,
    _state: State<RunningBotState>,
) -> Result<Action, anyhow::Error> {
    Ok(Action::ReplyText(get_version().into()))
}

async fn handle_any(e: Event, state: State<RunningBotState>) -> Result<Action, anyhow::Error> {
    match e.update {
        Update::Message(message) => {
            let message_content = message.text.unwrap();
            let state = state.get().read().await;
            let db = state
                .db_pool
                .as_ref()
                .cloned()
                .ok_or_else(|| anyhow!("Database pool not available"))?;

            let chat_bot = chat_bot::get_or_create_chat_bot(&db, message.chat.id)
                .await
                .context("Failed to get or create chat bot")?;

            let user_chat_state_read_lock = state.user_chat_state.read().await;
            let user_chat_state_value = user_chat_state_read_lock
                .get(&chat_bot.id)
                .unwrap_or(&UserChatState::Default)
                .clone();

            drop(user_chat_state_read_lock);

            match user_chat_state_value {
                UserChatState::WaitingBehaviorInput => {
                    chat_bot::set_chat_bot_behavior(&db, chat_bot.id, &message_content)
                        .await
                        .context("Failed to set the new chat bot behavior.")?;

                    let mut user_chat_state_write_lock = state.user_chat_state.write().await;
                    user_chat_state_write_lock.insert(chat_bot.id, UserChatState::Default);

                    Ok(Action::ReplyText(format!(
                        "Defined the new bot behavior as: '{}'",
                        message_content
                    )))
                }
                UserChatState::Default => {
                    let current_chat_thread =
                        chat_thread::get_or_create_chat_thread(&db, message.chat.id)
                            .await
                            .context("Failed to get the current chat thread")?;

                    let _new_chat_message = chat_message::insert_new_message(
                        &db,
                        &message_content,
                        message.chat.id,
                        current_chat_thread.id,
                        "user",
                    )
                    .await
                    .context("Failed to insert a new chat message")?;

                    let thread_messages = llm_thread_message::build_llm_thread_payload(
                        &db,
                        message.chat.id,
                        current_chat_thread.id,
                    )
                    .await
                    .context("Failed to get LLM payload.")?;

                    let llm_api_client: Box<dyn llm::LLMService> = match state.config.llm_service {
                        llm::LLMServiceKind::OpenAI => Box::new(llm::openai::OpenAI {
                            completion_model: llm::openai::OpenAICompletionModel::Gpt3_5turbo,
                        }),
                        _ => Box::new(llm::mock::Mock {
                            completion_model: llm::mock::MockCompletionModel::Bright,
                        }),
                    };

                    let maybe_answer = llm_api_client.get_answer(thread_messages);

                    match maybe_answer.await {
                        Ok(content) => {
                            let _new_chat_message = chat_message::insert_new_message(
                                &db,
                                &content,
                                message.chat.id,
                                current_chat_thread.id,
                                "assistant",
                            )
                            .await
                            .context("Failed to insert a new chat message")?;

                            Ok(Action::ReplyText(content))
                        }
                        Err(e) => Ok(Action::ReplyText(format!("Error: {:?}", e))),
                    }
                }
            }
        }
        Update::EditedMessage(message) => Ok(Action::ReplyText(format!(
            "Edited message: {}",
            message.text.unwrap()
        ))),
        _ => Ok(Action::ReplyText("Anyhow!".into())),
    }
}

async fn handle_start_new_thread(e: Event, state: State<RunningBotState>) -> Result<Action> {
    let message = e.update.get_new().context("Failed to get new update")?;
    let chat_id = message.chat.id;
    let state = state.get().read().await;
    let db = state
        .db_pool
        .as_ref()
        .cloned()
        .ok_or_else(|| anyhow!("Database pool not available"))?;

    let chat_bot = chat_bot::get_or_create_chat_bot(&db, chat_id)
        .await
        .context("Failed to get or create chat bot")?;

    let chat_thread = chat_thread::close_chat_thread(&db, chat_id)
        .await
        .context("Failed to get or create chat thread")?;

    match chat_thread {
        Some(chat_thread_id) => Ok(Action::ReplyText(format!(
            "The thread number {:?} for bot {:?} has been closed. Start a new one!",
            chat_thread_id, chat_bot.id
        ))),
        None => Ok(Action::ReplyText(format!(
            "The bot with id {:?} doesn't have active threads. Start a new one!",
            chat_bot.id
        ))),
    }
}

async fn handle_get_behavior(e: Event, state: State<RunningBotState>) -> Result<Action> {
    let message = e.update.get_new().context("Failed to get new update")?;
    let state = state.get().read().await;
    let db = state
        .db_pool
        .as_ref()
        .cloned()
        .ok_or_else(|| anyhow!("Database pool not available"))?;

    let chat_bot = chat_bot::get_or_create_chat_bot(&db, message.chat.id)
        .await
        .context("Failed to get or create chat bot")?;

    Ok(Action::ReplyText(format!(
        "The current bot behavior is defined as follows: '{:?}'. Use the /set_behavior command to change it.",
        chat_bot.behavior
    )))
}

async fn handle_set_behavior(e: Event, state: State<RunningBotState>) -> Result<Action> {
    let message = e.update.get_new().context("Failed to get new update")?;
    let state = state.get().read().await;
    let db = state
        .db_pool
        .as_ref()
        .cloned()
        .ok_or_else(|| anyhow!("Database pool not available"))?;

    let chat_bot = chat_bot::get_or_create_chat_bot(&db, message.chat.id)
        .await
        .context("Failed to get or create chat bot")?;

    let mut user_chat_state_write_lock = state.user_chat_state.write().await;
    user_chat_state_write_lock.insert(chat_bot.id, UserChatState::WaitingBehaviorInput);

    Ok(Action::ReplyText(format!(
        "Please enter the desired chat bot behavior in the next message. Example: 'You are a helpful assistant.'"
    )))
}

pub async fn start_bot(db_pool: &Pool<Sqlite>, config: Config) {
    let client = Client::new(config.telegram_token.to_string().into());
    let user_chat_state: Arc<RwLock<HashMap<i64, UserChatState>>> =
        Arc::new(RwLock::new(HashMap::new()));

    let state = RunningBotState {
        db_pool: Some(db_pool.clone()),
        user_chat_state,
        config,
    };

    let mut router = Router::<RunningBotState>::new(client).with_state(state);

    router.add_route(
        Route::Message(Matcher::Exact("/version".into())),
        handle_get_version,
    );
    router.add_route(
        Route::Message(Matcher::Exact("/get_behavior".into())),
        handle_get_behavior,
    );
    router.add_route(
        Route::Message(Matcher::Exact("/set_behavior".into())),
        handle_set_behavior,
    );
    router.add_route(
        Route::Message(Matcher::Exact("/new".into())),
        handle_start_new_thread,
    );
    router.add_route(Route::Message(Matcher::Any), handle_any);
    router.start().await;
}

pub fn get_version() -> String {
    format!(
        "{}.{}.{}",
        env!("CARGO_PKG_VERSION_MAJOR"),
        env!("CARGO_PKG_VERSION_MINOR"),
        env!("CARGO_PKG_VERSION_PATCH"),
    )
}
