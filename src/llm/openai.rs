use crate::llm::LLMService;
use crate::llm::LLMThreadMessage;
use async_trait::async_trait;
use std::error::Error;

use async_openai::{
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestMessageArgs,
        CreateChatCompletionRequestArgs, Role,
    },
    Client,
};

pub struct OpenAI;

#[async_trait]
impl LLMService for OpenAI {
    async fn get_answer(
        &self,
        thread_messages: Vec<LLMThreadMessage>,
    ) -> Result<String, Box<dyn Error>> {
        let client = Client::new();
        let mut chat_req_messages: Vec<ChatCompletionRequestMessage> = vec![];

        for msg in thread_messages {
            let comp_req = ChatCompletionRequestMessageArgs::default()
                .role(to_role(&msg.role))
                .content(msg.message)
                .build()?;

            chat_req_messages.push(comp_req);
        }

        let request = CreateChatCompletionRequestArgs::default()
            .max_tokens(512u16)
            .model("gpt-3.5-turbo")
            .messages(chat_req_messages)
            .build()?;

        let _response = client.chat().create(request).await?;

        Ok("Ok".to_owned())
    }
}

fn to_role(role: &String) -> Role {
    match role.as_str() {
        "system" => Role::System,
        "user" => Role::User,
        _ => Role::User,
    }
}
