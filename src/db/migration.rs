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
          behavior TEXT NOT NULL
      );

      CREATE TABLE IF NOT EXISTS chat_threads (
          id INTEGER PRIMARY KEY NOT NULL,
          is_current BOOLEAN NOT NULL DEFAULT TRUE,
          chat_id INTEGER NOT NULL
      );

      CREATE UNIQUE INDEX IF NOT EXISTS idx_one_current_thread_per_chat ON chat_threads(chat_id, is_current = true);
      ",
    )
    .execute(db_conn)
    .await
    .unwrap();
}
