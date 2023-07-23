use crate::llm::LLMService;
use crate::llm::LLMThreadMessage;
use anyhow::{anyhow, Result};
use async_trait::async_trait;

use async_openai::{
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestMessageArgs,
        CreateChatCompletionRequestArgs, CreateChatCompletionResponse, Role,
    },
    Client,
};

pub struct OpenAI;

async fn fetch_it(
    chat_req_messages: Vec<ChatCompletionRequestMessage>,
) -> Result<CreateChatCompletionResponse, anyhow::Error> {
    let client = Client::new();

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model("gpt-3.5-turbo")
        .messages(chat_req_messages)
        .build()?;

    let response = client
        .chat()
        .create(request)
        .await
        .map_err(|e| anyhow::Error::new(e))?;

    Ok(response)
}

#[async_trait]
impl LLMService for OpenAI {
    async fn get_answer(&self, thread_messages: Vec<LLMThreadMessage>) -> anyhow::Result<String> {
        let mut chat_req_messages: Vec<ChatCompletionRequestMessage> = vec![];

        for msg in thread_messages {
            let comp_req = ChatCompletionRequestMessageArgs::default()
                .role(to_role(&msg.role))
                .content(msg.message)
                .build()?;

            chat_req_messages.push(comp_req);
        }

        let chat_completion_response = fetch_it(chat_req_messages).await;
        chat_completion_response.and_then(get_first_choice)
    }
}

fn get_first_choice(resp: CreateChatCompletionResponse) -> anyhow::Result<String> {
    resp.choices
        .first()
        .and_then(|choice| choice.message.content.clone())
        .ok_or_else(|| anyhow!("Message content not found"))
}

fn to_role(role: &String) -> Role {
    match role.as_str() {
        "system" => Role::System,
        "user" => Role::User,
        _ => Role::User,
    }
}
