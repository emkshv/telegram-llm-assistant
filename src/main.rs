// use sqlx::sqlite::SqlitePool;
use std::env;
mod bot;
mod db;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    println!("{:?}", bot::get_version());

    // let url = "sqlite:todos.db";
    let url = &env::var("DATABASE_URL")?;

    db::start(url).await;

    Ok(())
}
