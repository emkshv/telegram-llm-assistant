use crate::llm::LLMService;
use crate::llm::LLMThreadMessage;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use std::str::FromStr;

use async_openai::{
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestMessageArgs,
        CreateChatCompletionRequestArgs, CreateChatCompletionResponse, Role,
    },
    Client,
};

#[derive(Copy, Clone, Debug)]
pub enum OpenAICompletionModel {
    Gpt4,
    Gpt4_0613,
    Gpt4_32k,
    Gpt4_32k0613,
    Gpt3_5turbo,
    Gpt3_5turbo0613,
    Gpt3_5turbo16k0613,
}

impl OpenAICompletionModel {
    fn as_str(&self) -> &'static str {
        match *self {
            OpenAICompletionModel::Gpt4 => "gpt-4",
            OpenAICompletionModel::Gpt4_0613 => "gpt-4-0613",
            OpenAICompletionModel::Gpt4_32k => "gpt-4-32k",
            OpenAICompletionModel::Gpt4_32k0613 => "gpt-4-32k-0613",
            OpenAICompletionModel::Gpt3_5turbo => "gpt-3.5-turbo",
            OpenAICompletionModel::Gpt3_5turbo0613 => "gpt-3.5-turbo-0613",
            OpenAICompletionModel::Gpt3_5turbo16k0613 => "gpt-3.5-turbo-16k-0613",
        }
    }
}

impl FromStr for OpenAICompletionModel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gpt-4" => Ok(OpenAICompletionModel::Gpt4),
            "gpt-4-0613" => Ok(OpenAICompletionModel::Gpt4_0613),
            "gpt-4-32k" => Ok(OpenAICompletionModel::Gpt4_32k),
            "gpt-4-32k-0613" => Ok(OpenAICompletionModel::Gpt4_32k0613),
            "gpt-3.5-turbo" => Ok(OpenAICompletionModel::Gpt3_5turbo),
            "gpt-3.5-turbo-0613" => Ok(OpenAICompletionModel::Gpt3_5turbo0613),
            "gpt-3.5-turbo-16k-0613" => Ok(OpenAICompletionModel::Gpt3_5turbo16k0613),
            _ => Err(()),
        }
    }
}

pub struct OpenAI {
    pub completion_model: OpenAICompletionModel,
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

        let chat_completion_response =
            fetch_response(chat_req_messages, self.completion_model).await;
        chat_completion_response.and_then(get_first_choice)
    }
}

async fn fetch_response(
    chat_req_messages: Vec<ChatCompletionRequestMessage>,
    completion_model: OpenAICompletionModel,
) -> Result<CreateChatCompletionResponse, anyhow::Error> {
    let client = Client::new();

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model(completion_model.as_str())
        .messages(chat_req_messages)
        .build()?;

    let response = client
        .chat()
        .create(request)
        .await
        .map_err(|e| anyhow::Error::new(e))?;

    Ok(response)
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
