use crate::llm::{
    groq::GroqCompletionModel, mock::MockCompletionModel, openai::OpenAICompletionModel,
};
use anyhow::{Context, Result};
use sqlx::{FromRow, Pool, Sqlite};

#[derive(Clone, FromRow, Debug)]
pub struct ChatBot {
    pub id: i64,
    pub behavior: String,
    pub mock_model: String,
    pub openai_model: String,
    pub groq_model: String,
}

async fn get_by_id(db_conn: &Pool<Sqlite>, id: i64) -> Result<ChatBot> {
    let chat_bot = sqlx::query_as::<_, ChatBot>("SELECT * FROM chat_bots WHERE id = ?1 LIMIT 1")
        .bind(id)
        .fetch_one(db_conn)
        .await?;

    Ok(chat_bot)
}

pub async fn get_or_create_chat_bot(db_conn: &Pool<Sqlite>, id: i64) -> Result<ChatBot> {
    let behavior: String = "You're a helpful assistant.".into();
    let mock_completion_model = MockCompletionModel::default_string();
    let openai_completion_model = OpenAICompletionModel::default_string();
    let groq_completion_model = GroqCompletionModel::default_string();

    sqlx::query(
        r#"INSERT INTO chat_bots
          (id, behavior, mock_model, openai_model, groq_model)
        VALUES(?1, ?2, ?3, ?4, ?5)
          ON CONFLICT (id)
          DO NOTHING"#,
    )
    .bind(id)
    .bind(behavior)
    .bind(mock_completion_model)
    .bind(openai_completion_model)
    .bind(groq_completion_model)
    .execute(db_conn)
    .await?;

    let chat_bot = get_by_id(db_conn, id).await?;

    Ok(chat_bot)
}

pub async fn set_chat_bot_behavior(
    db_conn: &Pool<Sqlite>,
    id: i64,
    behavior: &String,
) -> Result<ChatBot> {
    sqlx::query("UPDATE chat_bots SET behavior = ?1 WHERE id = ?2")
        .bind(behavior)
        .bind(id)
        .execute(db_conn)
        .await?;

    let chat_bot = get_by_id(db_conn, id).await?;

    Ok(chat_bot)
}

pub async fn set_chat_bot_mock_model(
    db_conn: &Pool<Sqlite>,
    id: i64,
    completion_model: MockCompletionModel,
) -> Result<ChatBot> {
    let completion_model_string = completion_model.as_str();

    let chat_bot = sqlx::query_as::<_, ChatBot>(
        "UPDATE chat_bots SET mock_model = ?1 WHERE id = ?2 RETURNING *;",
    )
    .bind(completion_model_string)
    .bind(id)
    .fetch_one(db_conn)
    .await
    .context(format!(
        "Couldn't update the chat bot's Mock completion model with {}",
        completion_model_string
    ))?;

    Ok(chat_bot)
}

pub async fn set_chat_bot_openai_model(
    db_conn: &Pool<Sqlite>,
    id: i64,
    completion_model: OpenAICompletionModel,
) -> Result<ChatBot> {
    let completion_model_string = completion_model.as_str();

    sqlx::query_as::<_, ChatBot>(
        "UPDATE chat_bots SET openai_model = ?1 WHERE id = ?2 RETURNING *;",
    )
    .bind(completion_model_string)
    .bind(id)
    .fetch_one(db_conn)
    .await
    .context(format!(
        "Couldn't update the chat bot's OpenAI completion model with {}",
        completion_model_string
    ))?;

    let chat_bot = get_by_id(db_conn, id).await?;

    Ok(chat_bot)
}

pub async fn set_chat_bot_groq_model(
    db_conn: &Pool<Sqlite>,
    id: i64,
    completion_model: GroqCompletionModel,
) -> Result<ChatBot> {
    let completion_model_string = completion_model.as_str();

    sqlx::query_as::<_, ChatBot>("UPDATE chat_bots SET groq_model = ?1 WHERE id = ?2 RETURNING *;")
        .bind(completion_model_string)
        .bind(id)
        .fetch_one(db_conn)
        .await
        .context(format!(
            "Couldn't update the chat bot's Groq completion model with {}",
            completion_model_string
        ))?;

    let chat_bot = get_by_id(db_conn, id).await?;

    Ok(chat_bot)
}
