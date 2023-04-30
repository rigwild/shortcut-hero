use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::actions::openai::OpenAIAction;
use crate::config::Config;

mod openai;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "name", rename_all = "snake_case")]
pub enum Action {
    #[serde(rename = "openai_ask_chatgpt")]
    OpenAIAskChatGPT { pre_prompt: String },
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::OpenAIAskChatGPT { .. } => {
                write!(f, "{}", "Ask ChatGPT")
            }
        }
    }
}

impl Action {
    pub fn run(&self, config: &Config, input_str: &str) -> anyhow::Result<String> {
        match self {
            Action::OpenAIAskChatGPT { pre_prompt } => {
                OpenAIAction::ask_chat_gpt(config, pre_prompt, input_str)
            }
        }
    }
}
