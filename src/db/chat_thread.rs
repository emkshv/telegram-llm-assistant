use sqlx::{FromRow, Pool, Sqlite};
extern crate rand;
use rand::Rng;

#[derive(Clone, FromRow, Debug)]
pub struct ChatThread {
    pub id: i64,
    pub is_current: bool,
    pub chat_id: i64,
}

pub async fn close_chat_thread(
    db_conn: &Pool<Sqlite>,
    chat_id: i64,
) -> anyhow::Result<Option<i64>> {
    let record = sqlx::query!(
        r#"UPDATE chat_threads SET is_current = false WHERE chat_id = ?1 AND is_current = true RETURNING id as "id?""#,
        chat_id
    )
    .fetch_optional(db_conn)
    .await?;

    Ok(record.and_then(|r| r.id))
}

pub async fn get_or_create_chat_thread(
    db_conn: &Pool<Sqlite>,
    chat_id: i64,
) -> anyhow::Result<ChatThread> {
    let chat_thread_optional = sqlx::query_as::<_, ChatThread>(
        "
      SELECT * FROM chat_threads WHERE chat_id = ?1 and is_current = true
    ",
    )
    .bind(chat_id)
    .fetch_optional(db_conn)
    .await?;

    match chat_thread_optional {
        Some(chat_thread) => Ok(chat_thread),
        None => {
            let new_id: i64 = rand::thread_rng().gen_range(1..i64::MAX);

            let chat_thread = sqlx::query_as::<_, ChatThread>(
                "INSERT INTO chat_threads (id, chat_id, is_current) VALUES(?1, ?2, ?3) RETURNING *",
            )
            .bind(new_id)
            .bind(chat_id)
            .bind(true)
            .fetch_one(db_conn)
            .await?;

            Ok(chat_thread)
        }
    }
}
