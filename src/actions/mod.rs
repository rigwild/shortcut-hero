use native_dialog::{MessageDialog, MessageType};
use serde::{Deserialize, Serialize};

use crate::actions::clipboard::ClipboardAction;
use crate::actions::openai::OpenAIAction;
use crate::config::Config;

mod clipboard;
mod openai;

/// Available actions to run.
///
/// The actions will be run in the order they are provided.
/// The result of each action will be provided to the next action.
///
/// Default input is an empty string.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "name", rename_all = "snake_case")]
pub enum Action {
    /// Print the configuration and the provided input. Returns input.
    Debug,
    /// Provide some input directly from the configuration file. Returns input.
    FixedInput { input: String },
    /// Print the input to the console. Returns input.
    PrintConsole,
    /// Show the input in a native OS dialog box. Returns input.
    ShowDialog {
        /// Title of the dialog box. Default is `Action Result`.
        #[serde(default = "action_result_str")]
        title: String,
    },
    /// Read the content of the clipboard. Returns content of the clipboard.
    ReadClipboard,
    /// Write the input to the clipboard. Returns input.
    WriteClipboard,
    #[serde(rename = "openai_ask_chatgpt")]
    /// Ask the provided input to ChatGPT. Returns the answer from ChatGPT.
    OpenAIAskChatGPT {
        /// Pre-prompt to provide to ChatGPT to customize the action. Default is nothing.
        #[serde(default)]
        pre_prompt: String,
    },
}

impl Action {
    pub fn run(&self, config: &Config, input_str: &str) -> anyhow::Result<String> {
        match self {
            Action::Debug => {
                print!("\n\n");
                println!("###########");
                println!("## DEBUG ##");
                println!("###########");
                print!("\nconfig={config:#?}\n\ninput_str={input_str:#?}");
                print!("\n\n###########\n\n");
                Ok(input_str.to_string())
            }
            Action::FixedInput { input } => Ok(input.to_string()),
            Action::PrintConsole => {
                println!("{input_str}");
                Ok(input_str.to_string())
            }
            Action::ShowDialog { title } => {
                MessageDialog::new()
                    .set_type(MessageType::Info)
                    .set_title(title)
                    .set_text(input_str)
                    .show_alert()
                    .unwrap();
                Ok(input_str.to_string())
            }
            Action::ReadClipboard => ClipboardAction::get_clipboard_content(),
            Action::WriteClipboard => {
                ClipboardAction::set_clipboard_content(input_str).unwrap();
                Ok(input_str.to_string())
            }
            Action::OpenAIAskChatGPT { pre_prompt } => {
                OpenAIAction::ask_chat_gpt(config, pre_prompt, input_str)
            }
        }
    }
}

fn action_result_str() -> String {
    "Action Result".to_string()
}
