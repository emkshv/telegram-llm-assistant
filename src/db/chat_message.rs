use sqlx::{FromRow, Pool, Sqlite};
extern crate rand;
use anyhow::Context;
use rand::Rng;

#[derive(Clone, FromRow, Debug)]
pub struct ChatMessage {
    pub id: i64,
    pub content: String,
    pub chat_id: i64,
    pub chat_thread_id: i64,
    pub user_role: String,
    pub inserted_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone, FromRow, Debug)]
pub struct NewChatMessage {
    pub id: i64,
    pub content: String,
    pub chat_id: i64,
    pub chat_thread_id: i64,
    pub user_role: String,
}

pub async fn insert_new_message(
    db_conn: &Pool<Sqlite>,
    content: &String,
    chat_id: i64,
    chat_thread_id: i64,
    user_role: &str,
) -> anyhow::Result<i64> {
    let new_id: i64 = rand::thread_rng().gen_range(1..i64::MAX);

    let _chat_message = sqlx::query_as!(
        NewChatMessage,
        r#"INSERT INTO chat_messages (id, content, chat_id, chat_thread_id, user_role) VALUES(?1, ?2, ?3, ?4, ?5)"#,
        new_id,
        content,
        chat_id,
        chat_thread_id,
        user_role
    )
    .execute(db_conn)
    .await
    .context("Failed to create a chat message")?;

    Ok(new_id)
}

pub async fn get_chat_thread_messages(
    db_conn: &Pool<Sqlite>,
    chat_thread_id: i64,
) -> anyhow::Result<Vec<ChatMessage>> {
    let chat_messages: Vec<ChatMessage> = sqlx::query_as(
        "SELECT * FROM chat_messages WHERE chat_thread_id = ? ORDER BY inserted_at ASC",
    )
    .bind(chat_thread_id)
    .fetch_all(db_conn)
    .await
    .context(format!(
        "Failed to ge tthe chat messages for id {}",
        chat_thread_id
    ))?;

    Ok(chat_messages)
}
