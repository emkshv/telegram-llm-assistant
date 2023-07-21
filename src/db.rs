use sqlx::{FromRow, Pool, Sqlite, SqlitePool};

pub mod chat_bot;
pub mod chat_thread;
pub mod migration;

#[derive(Clone, FromRow, Debug)]
pub struct ChatThread {
    pub id: i64,
    pub is_current: bool,
    pub chat_id: i64,
}

pub async fn start(url: &String) -> Pool<Sqlite> {
    migration::create_db_if_doesnt_exists(url).await;

    let db_conn = SqlitePool::connect(url).await.unwrap();
    migration::run_all_migrations(&db_conn).await;

    db_conn
}
