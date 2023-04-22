mod openai;

use crate::actions::openai::AskChatGPTAction;
use crate::config::Config;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "name")]
pub enum Action {
    AskChatGPT { pre_prompt: String },
}

impl Action {
    pub fn run(&self, config: &Config, input_str: &str) -> anyhow::Result<String> {
        match self {
            Action::AskChatGPT { pre_prompt } => {
                AskChatGPTAction::run(config, pre_prompt, input_str)
            }
        }
    }
}
