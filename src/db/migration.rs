use sqlx::{migrate::MigrateDatabase, Pool, Sqlite};

pub async fn create_db_if_doesnt_exists(url: &String) {
    let db_exists = Sqlite::database_exists(url).await.unwrap_or(false);

    if db_exists == false {
        println!("DB {} not found. Create a new one.", url);
        let res = Sqlite::create_database(url).await;

        match res {
            Ok(_) => println!("Created DB {}", url),
            Err(error) => println!("Error creating DB, {}", error),
        }
    }
}

pub async fn run_all_migrations(db_conn: &Pool<Sqlite>) {
    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS chat_bots (
                  id INTEGER PRIMARY KEY NOT NULL,
                  behavior TEXT NOT NULL,
                  openai_model TEXT NOT NULL,
                  groq_model TEXT NOT NULL,
                  mock_model TEXT NOT NULL
              );

        CREATE UNIQUE INDEX IF NOT EXISTS unique_index_chat_bot_ids
              ON chat_bots (id);

        CREATE TABLE IF NOT EXISTS chat_threads (
                  id INTEGER PRIMARY KEY NOT NULL,
                  is_current BOOLEAN,
                  chat_id INTEGER NOT NULL
              );

        CREATE UNIQUE INDEX IF NOT EXISTS idx_one_current_thread_per_chat
              ON chat_threads(chat_id) WHERE is_current;

        CREATE TABLE IF NOT EXISTS chat_messages (
            id INTEGER PRIMARY KEY NOT NULL,
            content TEXT NOT NULL,
            chat_id INTEGER NOT NULL,
            chat_thread_id INTEGER NOT NULL,
            user_role TEXT NOT NULL,
            inserted_at DATETIME DEFAULT(STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW'))
        );

        CREATE INDEX IF NOT EXISTS idx_chat_messages_inserted_at
              ON chat_messages (inserted_at);
      ",
    )
    .execute(db_conn)
    .await
    .unwrap();
}
