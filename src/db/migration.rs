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

async fn migration_create_chat_bots(db_conn: &Pool<Sqlite>) {
    let result = sqlx::query(
        "
      CREATE TABLE IF NOT EXISTS chat_bots (
          id INTEGER PRIMARY KEY NOT NULL,
          description TEXT NOT NULL
      );

      CREATE UNIQUE INDEX IF NOT EXISTS unique_index_chat_bot_ids
      ON chat_bots (id);
      ",
    )
    .execute(db_conn)
    .await
    .unwrap();

    println!("Create chat_bots result: {:?}", result);
}

pub async fn run_all_migrations(db_conn: &Pool<Sqlite>) {
    migration_create_chat_bots(db_conn).await;
}
