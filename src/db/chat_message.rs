use sqlx::{FromRow, Pool, Sqlite};
extern crate rand;
use rand::Rng;

#[derive(Clone, FromRow, Debug)]
pub struct ChatMessage {
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
) -> anyhow::Result<ChatMessage> {
    let new_id: i64 = rand::thread_rng().gen_range(1..i64::MAX);

    let chat_message = sqlx::query_as::<_, ChatMessage>(
        "INSERT INTO chat_messages (id, content, chat_id, chat_thread_id, user_role) VALUES(?1, ?2, ?3, ?4, ?5) RETURNING *",
    )
    .bind(new_id)
    .bind(content)
    .bind(chat_id)
    .bind(chat_thread_id)
    .bind(user_role)
    .fetch_one(db_conn)
    .await?;

    Ok(chat_message)
}
