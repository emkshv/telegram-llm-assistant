use std::env;

mod bot;
mod config;
mod db;
mod llm;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let config = config::create_config();

    println!(
        "Stared the telegram bot assistant. Version {:?}.",
        bot::get_version()
    );

    let db_url = env::var("BOT_SQLITE_DATABASE_URL");

    match db_url {
        Ok(db_url_val) => {
            let db_pool = db::start(&db_url_val).await;
            bot::start_bot(&db_pool, config).await;
        }
        Err(e) => println!("Please, set {:?} environment variable to continue.", e),
    }

    Ok(())
}
