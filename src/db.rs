use sqlx::{migrate::MigrateDatabase, Executor, FromRow, Pool, Row, Sqlite, SqlitePool};
extern crate rand;
use rand::Rng;

// let new_id: i64 = rand::thread_rng().gen_range(1..i64::MAX);

#[derive(Clone, FromRow, Debug)]
struct ChatBot {
    id: i64,
    description: String,
}

async fn create_db_if_doesnt_exists(url: &String) {
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

async fn run_migrations(db_conn: &Pool<Sqlite>) {
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

async fn upsert_chat_bot(db_conn: &Pool<Sqlite>, id: i64, desc: String) -> anyhow::Result<i64> {
    sqlx::query!(
        "
          INSERT INTO chat_bots (id, description) VALUES (?1, ?2)
            ON CONFLICT(id) DO UPDATE SET description = ?2;
        ",
        id,
        desc
    )
    .execute(db_conn)
    .await?;

    Ok(id)
}

async fn get_or_create_chat_bot(db_conn: &Pool<Sqlite>, id: i64) -> anyhow::Result<ChatBot> {
    let def_desc: String = "You're helpful!".into();

    sqlx::query!(
        "INSERT INTO chat_bots (id, description) VALUES(?1, ?2) ON CONFLICT (id) DO NOTHING;",
        id,
        def_desc
    )
    .execute(db_conn)
    .await?;

    let chat_bot =
        sqlx::query_as::<_, ChatBot>("SELECT id, description FROM chat_bots WHERE id = ?1")
            .bind(id)
            .fetch_one(db_conn)
            .await?;

    Ok(chat_bot)
}

pub async fn start(url: &String) {
    println!("start db");
    create_db_if_doesnt_exists(url).await;

    let db_conn = SqlitePool::connect(url).await.unwrap();
    run_migrations(&db_conn).await;

    let new_id: i64 = rand::thread_rng().gen_range(1..i64::MAX);
    let desc: String = "You are a helpful assistant?".into();
    let chat_bot_id = upsert_chat_bot(&db_conn, new_id, desc).await;

    match chat_bot_id {
        Ok(id) => println!("new chat bot {:?}", id),
        Err(e) => println!("hm {:?}", e),
    }

    let new_id2: i64 = rand::thread_rng().gen_range(1..i64::MAX);
    let cb = get_or_create_chat_bot(&db_conn, new_id2).await;
    match cb {
        Ok(c) => println!("chat bot: {:?}", c),
        Err(e) => println!("whoops {:?}", e),
    }

    let cb = get_or_create_chat_bot(&db_conn, new_id2).await;
    match cb {
        Ok(c) => println!("chat bot: {:?}", c),
        Err(e) => println!("whoops {:?}", e),
    }

    println!("to be cont...");
}