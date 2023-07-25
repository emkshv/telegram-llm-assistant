pub mod llm_thread_message;
pub mod mock;
pub mod openai;
use async_trait::async_trait;
use clap::ValueEnum;
use std::fmt;

use llm_thread_message::LLMThreadMessage;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum LLMServiceKind {
    OpenAI,
    Mock,
}

impl fmt::Display for LLMServiceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            LLMServiceKind::OpenAI => "OpenAI",
            LLMServiceKind::Mock => "Mock",
        };

        write!(f, "{}", name)
    }
}

impl Default for LLMServiceKind {
    fn default() -> Self {
        LLMServiceKind::OpenAI
    }
}

#[async_trait]
pub trait LLMServiceModel: Sync + Send {}

#[async_trait]
pub trait LLMService: Send {
    async fn get_answer(&self, thread_messages: Vec<LLMThreadMessage>) -> anyhow::Result<String>;

    fn bot_info(&self) -> String;
}
