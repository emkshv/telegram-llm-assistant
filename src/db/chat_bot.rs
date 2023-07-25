use crate::llm::mock::MockCompletionModel;
use crate::llm::openai::OpenAICompletionModel;
use anyhow::{Context, Result};
use sqlx::{FromRow, Pool, Sqlite};

#[derive(Clone, FromRow, Debug)]
pub struct ChatBot {
    pub id: i64,
    pub behavior: String,
    pub mock_model: String,
    pub openai_model: String,
}

async fn get_by_id(db_conn: &Pool<Sqlite>, id: i64) -> Result<ChatBot> {
    let chat_bot = sqlx::query_as!(
        ChatBot,
        r#"SELECT * FROM chat_bots WHERE id = ?1 LIMIT 1;"#,
        id
    )
    .fetch_one(db_conn)
    .await
    .context(format!("Failed to get new chat bot by id {}", id))?;

    Ok(chat_bot)
}

pub async fn get_or_create_chat_bot(db_conn: &Pool<Sqlite>, id: i64) -> Result<ChatBot> {
    let def_desc: String = "You're a helpful assistant.".into();
    let mock_completion_model = MockCompletionModel::default_string();
    let openai_completion_model = OpenAICompletionModel::default_string();

    sqlx::query_as!(
        ChatBot,
        r#"INSERT INTO chat_bots (id, behavior, mock_model, openai_model) VALUES(?1, ?2, ?3, ?4) ON CONFLICT (id) DO NOTHING;"#,
        id,
        def_desc,
        mock_completion_model,
        openai_completion_model
    )
    .execute(db_conn)
    .await
    .context("Failed to create a new chat bot")?;

    let chat_bot = get_by_id(db_conn, id).await?;

    Ok(chat_bot)
}

pub async fn set_chat_bot_behavior(
    db_conn: &Pool<Sqlite>,
    id: i64,
    behavior: &String,
) -> Result<ChatBot> {
    sqlx::query!(
        r#"UPDATE chat_bots SET behavior = ?1 WHERE id = ?2;"#,
        behavior,
        id
    )
    .execute(db_conn)
    .await
    .context("Failed to update chat_bot's behavior")?;

    let chat_bot = get_by_id(db_conn, id).await?;

    Ok(chat_bot)
}

pub async fn set_chat_bot_mock_model(
    db_conn: &Pool<Sqlite>,
    id: i64,
    completion_model: MockCompletionModel,
) -> Result<ChatBot> {
    let completion_model_string = completion_model.as_str();

    sqlx::query!(
        r#"UPDATE chat_bots SET mock_model = ?1 WHERE id = ?2;"#,
        completion_model_string,
        id
    )
    .fetch_one(db_conn)
    .await
    .context(format!(
        "Couldn't update the chat bot's Mock completion model with {}",
        completion_model_string
    ))?;

    let chat_bot = get_by_id(db_conn, id).await?;

    Ok(chat_bot)
}

pub async fn set_chat_bot_openai_model(
    db_conn: &Pool<Sqlite>,
    id: i64,
    completion_model: OpenAICompletionModel,
) -> Result<ChatBot> {
    let completion_model_string = completion_model.as_str().to_string();

    sqlx::query!(
        r#"UPDATE chat_bots SET openai_model = ?1 WHERE id = ?2;"#,
        completion_model_string,
        id,
    )
    .fetch_one(db_conn)
    .await
    .context(format!(
        "Couldn't update the chat bot's OpenAI completion model with {}",
        completion_model_string
    ))?;

    let chat_bot = get_by_id(db_conn, id).await?;

    Ok(chat_bot)
}
