use mobot::*;
use std::env;

async fn handle_get_version(e: Event, state: State<()>) -> Result<Action, anyhow::Error> {
    Ok(Action::ReplyText(get_version().into()))
}

async fn handle_any(e: Event, state: State<()>) -> Result<Action, anyhow::Error> {
    match e.update {
        Update::Message(message) => Ok(Action::ReplyText(format!(
            "Got new message: {}",
            message.text.unwrap()
        ))),
        Update::EditedMessage(message) => Ok(Action::ReplyText(format!(
            "Edited message: {}",
            message.text.unwrap()
        ))),
        _ => Ok(Action::ReplyText("Anyhow!".into())),
    }
}

#[tokio::main]
pub async fn start_bot() {
    let telegram_token = env::var("TELEGRAM_TOKEN");

    println!("{:?}", telegram_token);
    let client = Client::new(telegram_token.unwrap().into());
    let mut router = Router::<()>::new(client);

    router.add_route(
        Route::Message(Matcher::Exact("version".into())),
        handle_get_version,
    );
    router.add_route(Route::Message(Matcher::Any), handle_any);
    router.add_route(Route::Default, |_, _| async move {
        Ok(Action::ReplyText("Hello world!".into()))
    });
    router.start().await;
}

pub fn get_version() -> String {
    format!(
        "{}.{}.{}",
        env!("CARGO_PKG_VERSION_MAJOR"),
        env!("CARGO_PKG_VERSION_MINOR"),
        env!("CARGO_PKG_VERSION_PATCH"),
    )
}
