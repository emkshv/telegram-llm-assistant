use std::env;
mod bot;
mod db;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    println!("{:?}", bot::get_version());

    // let url = "sqlite:todos.db";

    // let url = env::var("BOT_SQLITE_DATABASE_URL")?;
    // db::start(&url).await;

    let key = "BOT_SQLITE_DATABASE_URL";
    let db_url = env::var("BOT_SQLITE_DATABASE_URL");

    match db_url {
        Ok(val) => db::start(&val).await,
        Err(e) => println!("Please, set {:?} environment variable to continue.", key),
    }

    Ok(())
}
