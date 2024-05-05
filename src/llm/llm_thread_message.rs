use serde::{Deserialize, Serialize};

use anyhow::Context;
use sqlx::{Pool, Sqlite};

use crate::db::chat_bot;
use crate::db::chat_message;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LLMThreadMessage {
    pub message: String,
    pub role: String,
}

pub async fn build_llm_thread_payload(
    db_conn: &Pool<Sqlite>,
    chad_id: i64,
    chat_thread_id: i64,
) -> anyhow::Result<Vec<LLMThreadMessage>> {
    let chat_bot = chat_bot::get_or_create_chat_bot(db_conn, chad_id)
        .await
        .context("Failed to get or create chat bot")?;

    let initial_message = LLMThreadMessage {
        message: chat_bot.behavior,
        role: "system".to_string(),
    };

    let chat_thread_messages = chat_message::get_chat_thread_messages(db_conn, chat_thread_id)
        .await
        .context("Failed to get the thread")?;

    let mut payload_messages: Vec<LLMThreadMessage> = chat_thread_messages
        .iter()
        .map(|m| LLMThreadMessage {
            message: m.content.clone(),
            role: m.user_role.clone(),
        })
        .collect();

    payload_messages.insert(0, initial_message);

    Ok(payload_messages)
}
