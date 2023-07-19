use sqlx::{migrate::MigrateDatabase, Row, Sqlite, SqlitePool};

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

pub async fn start(url: &String) {
    println!("start db");
    create_db_if_doesnt_exists(url).await;
    println!("to be cont...");
}
