use sqlx::{FromRow, Pool, Sqlite, SqlitePool};
extern crate rand;
// use rand::Rng;

pub mod migration;

// let new_id: i64 = rand::thread_rng().gen_range(1..i64::MAX);

#[derive(Clone, FromRow, Debug)]
pub struct ChatBot {
    pub id: i64,
    pub behavior: String,
}

pub async fn get_or_create_chat_bot(db_conn: &Pool<Sqlite>, id: i64) -> anyhow::Result<ChatBot> {
    let def_desc: String = "You're a helpful assistant.".into();

    sqlx::query!(
        "INSERT INTO chat_bots (id, behavior) VALUES(?1, ?2) ON CONFLICT (id) DO NOTHING;",
        id,
        def_desc
    )
    .execute(db_conn)
    .await?;

    let chat_bot = sqlx::query_as::<_, ChatBot>("SELECT id, behavior FROM chat_bots WHERE id = ?1")
        .bind(id)
        .fetch_one(db_conn)
        .await?;

    Ok(chat_bot)
}

pub async fn set_chat_bot_behavior(
    db_conn: &Pool<Sqlite>,
    id: i64,
    behavior: &String,
) -> anyhow::Result<ChatBot> {
    let chat_bot = sqlx::query_as::<_, ChatBot>(
        "UPDATE chat_bots SET behavior = ?1 WHERE id = ?2 RETURNING *;",
    )
    .bind(behavior)
    .bind(id)
    .fetch_one(db_conn)
    .await?;

    Ok(chat_bot)
}

pub async fn start(url: &String) -> Pool<Sqlite> {
    migration::create_db_if_doesnt_exists(url).await;

    let db_conn = SqlitePool::connect(url).await.unwrap();
    migration::run_all_migrations(&db_conn).await;

    db_conn
}
