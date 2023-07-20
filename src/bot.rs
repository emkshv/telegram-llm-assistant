use crate::db;
use mobot::*;
use sqlx::{Pool, Sqlite};
use std::env;

#[derive(Clone, Default, BotState)]
struct ChatState {
    db_pool: Option<Pool<Sqlite>>,
}

async fn handle_get_version(_e: Event, _state: State<ChatState>) -> Result<Action, anyhow::Error> {
    Ok(Action::ReplyText(get_version().into()))
}

async fn handle_any(e: Event, _state: State<ChatState>) -> Result<Action, anyhow::Error> {
    match e.update {
        Update::Message(message) => Ok(Action::ReplyText(format!(
            "Got new message: {}",
            message.text.unwrap()
        ))),
        Update::EditedMessage(message) => Ok(Action::ReplyText(format!(
            "Edited message: {}",
            message.text.unwrap()
        ))),
        _ => Ok(Action::ReplyText("Anyhow!".into())),
    }
}

async fn handle_get_behavior(e: Event, state: State<ChatState>) -> Result<Action, anyhow::Error> {
    let message = e.update.get_new()?;
    let state = state.get().read().await;
    let db = state.db_pool.as_ref().cloned();

    match db {
        Some(indeed_db) => {
            let chat_bot = db::get_or_create_chat_bot(&indeed_db, message.chat.id).await;

            match chat_bot {
                Ok(cb) => {
                    Ok(Action::ReplyText(format!(
                        "Current bot behavior is defined as follows: '{:?}'. Use the /set_behavior command to change it.",
                        cb.description
                    )))
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                    Ok(Action::ReplyText(format!("Something went wrong!")))
                }
            }
        }
        None => Ok(Action::ReplyText(format!("Something went wrong!"))),
    }
}

pub async fn start_bot(db_pool: &Pool<Sqlite>) {
    let telegram_token = env::var("TELEGRAM_TOKEN");

    let client = Client::new(telegram_token.unwrap().into());

    let state = ChatState {
        db_pool: Some(db_pool.clone()),
    };

    let mut router = Router::<ChatState>::new(client).with_state(state);

    router.add_route(
        Route::Message(Matcher::Exact("/version".into())),
        handle_get_version,
    );
    router.add_route(
        Route::Message(Matcher::Exact("/show_behavior".into())),
        handle_get_behavior,
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
