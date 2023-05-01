use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::actions::basic::BasicAction;
use crate::actions::clipboard::ClipboardAction;
use crate::actions::openai::OpenAIAction;
use crate::config::Config;

mod basic;
mod clipboard;
mod openai;

/// Actions are synchronous functions that take some input and return some output, they can do
/// anything.
///
/// The actions will run in the order they are defined. The result of each action is provided
/// to the next action as an input (variable `input`).
///
/// If an action requires any parameter, you can use variables enclosed in tags `{{input}}` or
/// `{{my_variable}}`, they will be replaced everywhere with the associated value.
///
/// Variables names are case-insensitive.
///
/// The first action in the list will receive an empty string as an input.
///
/// You may want to start
/// your list of actions with an action that read some data as input for the next actions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum Action {
    /// Print the configuration, the provided input and the list of variables. Returns input.
    Debug,

    /// Set the value of a variable. Returns input.
    SetVariable {
        /// Name of the variable to set.
        name: String,
        /// Value of the variable.
        value: String,
    },
    /// Delete a variable. Returns input (if the deleted variable is `input`, returns nothing).
    DeleteVariable {
        /// Name of the variable to clear.
        name: String,
    },

    /// Print the input to the console. Returns input.
    PrintConsole {
        /// Content to print in the console. Default is `{{input}}`.
        #[serde(default = "input_tag_str")]
        content: String,
    },
    /// Show the input in a native OS dialog box. Returns input.
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
    /// Write to the clipboard. Returns input.
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

    /// Ask something to ChatGPT. Returns the answer from ChatGPT.
    #[serde(rename = "ask_chatgpt")]
    AskChatGPT {
        /// Pre-prompt to provide to ChatGPT to customize the action. Default is empty.
        #[serde(default)]
        pre_prompt: String,
        /// Prompt to provide to ChatGPT. Default is {{input}}.
        #[serde(default = "input_tag_str")]
        prompt: String,
    },
}

impl Action {
    pub fn run(
        &self,
        config: &Config,
        input_str: &str,
        variables: &mut HashMap<String, String>,
    ) -> anyhow::Result<String> {
        match self {
            Action::Debug => {
                print!("\n\n");
                println!("###########");
                println!("## DEBUG ##");
                println!("###########");
                print!(
                    "\nconfig={config:#?}\n\ninput_str={input_str:#?}\nvariables={variables:#?}"
                );
                print!("\n\n###########\n\n");
                Ok(input_str.to_string())
            }

            Action::SetVariable { name, value } => {
                if name.to_lowercase() == "input" {
                    Ok(value.to_string())
                } else {
                    variables.insert(
                        name.to_lowercase().to_string(),
                        replace_variables_tag(value, input_str, variables),
                    );
                    Ok(input_str.to_string())
                }
            }
            Action::DeleteVariable { name } => {
                if name.to_lowercase() == "input" {
                    Ok("".to_string())
                } else {
                    variables.remove(name.to_lowercase().as_str());
                    Ok(input_str.to_string())
                }
            }

            Action::PrintConsole { content } => {
                BasicAction::print_console(&replace_variables_tag(content, input_str, variables))?;
                Ok(input_str.to_string())
            }
            Action::ShowDialog { title, body } => {
                BasicAction::show_dialog(
                    &replace_variables_tag(title, input_str, variables),
                    &replace_variables_tag(body, input_str, variables),
                )?;
                Ok(input_str.to_string())
            }

            Action::ReadClipboard => ClipboardAction::get_clipboard_content(),
            Action::WriteClipboard { content } => {
                ClipboardAction::set_clipboard_content(&replace_variables_tag(
                    content, input_str, variables,
                ))?;
                Ok(input_str.to_string())
            }

            Action::Spawn { command, args } => BasicAction::spawn(
                &replace_variables_tag(command, input_str, variables),
                &replace_variables_tag_vec(args, input_str, variables),
            ),

            Action::AskChatGPT { pre_prompt, prompt } => OpenAIAction::ask_chat_gpt(
                config,
                &replace_variables_tag(pre_prompt, input_str, variables),
                &replace_variables_tag(prompt, input_str, variables),
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

fn replace_variables_tag(
    str: &str,
    input_str: &str,
    variables: &HashMap<String, String>,
) -> String {
    let mut str = str.to_string();
    str = str.replace("{{input}}", input_str);
    for (key, value) in variables {
        str = str.replace(&format!("{{{{{}}}}}", key.to_lowercase()), value);
    }
    str
}

fn replace_variables_tag_vec(
    vec_of_str: &Vec<String>,
    input_str: &str,
    variables: &HashMap<String, String>,
) -> Vec<String> {
    vec_of_str
        .iter()
        .map(|str| replace_variables_tag(str, input_str, variables))
        .collect()
}
