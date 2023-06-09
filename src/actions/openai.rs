use std::collections::HashMap;
use std::env;

use anyhow::anyhow;
use reqwest::blocking::Client as HttpClient;
use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::evaluation::replace_variables_tag;
use crate::hotkey::ShortcutResult;

pub struct OpenAIAction;

impl OpenAIAction {
    pub fn ask_chat_gpt(
        config: &Config,
        input_str: &str,
        variables: &HashMap<String, String>,
        pre_prompt: &str,
        prompt: &str,
    ) -> anyhow::Result<ShortcutResult> {
        let pre_prompt = replace_variables_tag(pre_prompt, input_str, variables);
        let prompt = replace_variables_tag(prompt, input_str, variables);
        let res = ask_chatgpt(&pre_prompt, &prompt, config.openai_api_key.as_str())?;
        Ok(ShortcutResult::Success(res))
    }
}

const OPENAI_MODEL: &'static str = "gpt-3.5-turbo";

fn ask_chatgpt(pre_prompt: &str, prompt: &str, api_key: &str) -> anyhow::Result<String> {
    if let Ok(response) = env::var("OPENAI_API_RESPONSE_MOCK") {
        return Ok(response);
    }
    // We don't use `Message::System` for the pre-prompt as OpenAI recommends to use a user message
    // for this model instead to get better results
    let messages = vec![Message::User(pre_prompt), Message::User(prompt)];
    request_chatgpt_api(messages, api_key)
}

fn request_chatgpt_api(messages: Vec<Message>, api_key: &str) -> anyhow::Result<String> {
    if let Ok(response) = env::var("OPENAI_API_RESPONSE_MOCK") {
        return Ok(response);
    }

    let body = ChatRequestInput {
        model: OPENAI_MODEL.to_string(),
        messages,
    };

    let client = HttpClient::new();
    let resp = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {api_key}"))
        .json(&body)
        .send()?;

    if resp.status().is_success() {
        let res: ChatResponse = resp.json()?;
        Ok(res.choices.get(0).unwrap().message.content.clone())
    } else {
        Err(anyhow!(
            "Error when calling the OpenAI chat completion API - Status: {} - Body: {}",
            resp.status(),
            resp.text().unwrap()
        ))
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "role", content = "content", rename_all = "lowercase")]
#[allow(dead_code)]
pub enum Message<'a> {
    System(&'a str),
    Assistant(&'a str),
    User(&'a str),
}

#[derive(Debug, Serialize)]
struct ChatRequestInput<'a> {
    model: String,
    messages: Vec<Message<'a>>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct ChatResponseUsage {
    pub prompt_tokens: i64,
    pub completion_tokens: i64,
    pub total_tokens: i64,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct ChatResponseMessage {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct ChatResponseChoice {
    pub index: i64,
    pub message: ChatResponseMessage,
    pub finish_reason: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct ChatResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub choices: Vec<ChatResponseChoice>,
    pub usage: ChatResponseUsage,
}
