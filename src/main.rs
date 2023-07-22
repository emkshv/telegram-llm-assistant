use std::env;
mod bot;
mod db;
mod llm;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    println!("{:?}", bot::get_version());

    let db_url = env::var("BOT_SQLITE_DATABASE_URL");

    match db_url {
        Ok(db_url_val) => {
            let db_pool = db::start(&db_url_val).await;
            bot::start_bot(&db_pool).await;
        }
        Err(e) => println!("Please, set {:?} environment variable to continue.", e),
    }

    Ok(())
}
