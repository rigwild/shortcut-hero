use serde::{Deserialize, Serialize};

use crate::actions::basic::BasicAction;
use crate::actions::clipboard::ClipboardAction;
use crate::actions::openai::OpenAIAction;
use crate::config::Config;

mod basic;
mod clipboard;
mod openai;

/// Available actions to run.
///
/// The actions will be run in the order they are provided.
/// The result of each action will be provided as an input to the next action.
///
/// The input tag `{{input}}` can be used to inject the input into any parameters of the actions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "name", rename_all = "snake_case")]
pub enum Action {
    /// Print the configuration and the provided input. Returns input.
    Debug,
    /// Remove the input, useful if the next action does not require an input. Returns empty string.
    ClearInput,
    /// Set the input. Returns the new input.
    SetInput {
        /// Content to set as the new input. Default is `{{input}}`.
        #[serde(default = "input_tag_str")]
        content: String,
    },
    /// Print the input to the console. Returns original input.
    PrintConsole {
        /// Content to print in the console. Default is `{{input}}`.
        #[serde(default = "input_tag_str")]
        content: String,
    },
    /// Show the input in a native OS dialog box. Returns original input.
    ShowDialog {
        /// Title of the dialog box. Default is `Action Result`.
        #[serde(default = "action_result_str")]
        title: String,
        /// Content to show inside the dialog. Default is `{{input}}`.
        #[serde(default = "input_tag_str")]
        body: String,
    },
    /// Read the content of the clipboard. Returns content of the clipboard.
    ReadClipboard,
    /// Write the input to the clipboard. Returns original input.
    WriteClipboard {
        /// Content to write to the clipboard. Default is `{{input}}`.
        #[serde(default = "input_tag_str")]
        content: String,
    },
    /// Spawn a system command. Returns the result of the command.
    Spawn {
        /// Command to execute.
        command: String,
        /// Arguments to pass to the command. Default is empty list.
        #[serde(default)]
        args: Vec<String>,
    },
    /// Ask the provided input to ChatGPT. Returns the answer from ChatGPT.
    #[serde(rename = "openai_ask_chatgpt")]
    OpenAIAskChatGPT {
        /// Pre-prompt to provide to ChatGPT to customize the action. Default is empty.
        #[serde(default)]
        pre_prompt: String,
        /// Prompt to provide to ChatGPT. Default is {{input}}.
        #[serde(default = "input_tag_str")]
        prompt: String,
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
            Action::ClearInput => Ok("".to_string()),
            Action::SetInput { content } => Ok(replace_input_tag(content, input_str)),

            Action::PrintConsole { content } => {
                BasicAction::print_console(&replace_input_tag(content, input_str))?;
                Ok(input_str.to_string())
            }
            Action::ShowDialog { title, body } => {
                BasicAction::show_dialog(
                    &replace_input_tag(title, input_str),
                    &replace_input_tag(body, input_str),
                )?;
                Ok(input_str.to_string())
            }
            Action::ReadClipboard => ClipboardAction::get_clipboard_content(),
            Action::WriteClipboard { content } => {
                ClipboardAction::set_clipboard_content(&replace_input_tag(content, input_str))?;
                Ok(input_str.to_string())
            }
            Action::Spawn { command, args } => BasicAction::spawn(
                &replace_input_tag(command, input_str),
                &replace_input_tag_vec(args, input_str),
            ),
            Action::OpenAIAskChatGPT { pre_prompt, prompt } => OpenAIAction::ask_chat_gpt(
                config,
                &replace_input_tag(pre_prompt, input_str),
                &replace_input_tag(prompt, input_str),
            ),
        }
    }
}

fn action_result_str() -> String {
    "Action Result".to_string()
}

fn input_tag_str() -> String {
    "{{input}}".to_string()
}

fn replace_input_tag(str: &str, input_str: &str) -> String {
    str.replace("{{input}}", input_str)
}

fn replace_input_tag_vec(vec_of_str: &Vec<String>, input_str: &str) -> Vec<String> {
    vec_of_str
        .iter()
        .map(|str| replace_input_tag(str, input_str))
        .collect()
}
