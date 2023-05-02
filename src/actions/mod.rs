use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::actions::basic::BasicAction;
use crate::actions::clipboard::ClipboardAction;
use crate::actions::core::CoreAction;
use crate::actions::openai::OpenAIAction;
use crate::config::Config;
use crate::hotkey::ShortcutResult;

mod basic;
mod clipboard;
mod core;
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
    /// Increment the value of a variable. Returns input.
    IncrementVariable {
        /// Name of the variable to increment.
        name: String,
        /// Integer value to increment the variable by, can be negative. Default is `1`.
        #[serde(default = "one_str")]
        amount: String,
    },
    /// Delete a variable. Returns input (if the deleted variable is `input`, returns nothing).
    DeleteVariable {
        /// Name of the variable to clear.
        name: String,
    },
    /// Wait for a given duration. Returns input.
    Sleep {
        /// Duration of the sleep in milliseconds.
        duration_ms: String,
    },
    /// End the program.
    EndProgram,
    /// Go to a given step in the list of actions (starts at 0). Returns input.
    ///
    /// Will error out if the step is out of bounds.
    GoToStep {
        /// Step index to go to in the list of actions (starts at 0).
        step: String,
    },
    /// Go to a given step in the list of actions relative from the current step. Returns input.
    ///
    /// Will error out if the step is out of bounds.
    ///
    /// ## Example
    ///
    /// - If the current step is 2 and the relative step is `1`, the next step will be `3`.
    /// - If the current step is 2 and the relative step is `-1`, the next step will be `1`.
    GoToStepRelative {
        /// Step index relative from current step to go to in the list of actions (starts at 0).
        step: String,
    },
    /// Go to a given step in the list of actions (starts at 0) or another depending on condition.
    ///
    /// Returns input.
    ///
    /// Will error out if the step is out of bounds.
    IfElse {
        /// Operation to perform on `A` and `B` to determine if the condition is true.
        ///
        /// - Real numbers comparisons:
        ///   - `==`
        ///   - `!=`
        ///   - `<`
        ///   - `<=`
        ///   - `>`
        ///   - `>=`
        ///
        /// - String comparisons:
        ///   - `str_equals`
        ///   - `str_not_equals
        ///   - `str_contains`
        ///   - `str_not_contains`,
        ///   - `str_starts_with`
        ///   - `str_ends_with`
        ///   - `str_is_empty` (only on `A`)
        ///   - `str_is_not_empty` (only on `A`)
        operation: String,
        /// Value A to compare.
        a: String,
        /// Value B to compare. Defaults to empty string (useful for operation only on `A`).
        #[serde(default)]
        b: String,
        /// Step index to go to in the list of actions (starts at 0) if condition is true.
        step_true: String,
        /// Step index to go to in the list of actions (starts at 0) if condition is false.
        step_false: String,
    },
    /// Go to a given step in the list of actions (starts at 0) or another depending on condition.
    ///
    /// Returns input.
    ///
    /// Will error out if the step is out of bounds.
    IfElseRelative {
        /// Operation to perform on `A` and `B` to determine if the condition is true.
        ///
        /// - Real numbers comparisons:
        ///   - `==`
        ///   - `!=`
        ///   - `<`
        ///   - `<=`
        ///   - `>`
        ///   - `>=`
        ///
        /// - String comparisons:
        ///   - `str_equals`
        ///   - `str_not_equals
        ///   - `str_contains`
        ///   - `str_not_contains`,
        ///   - `str_starts_with`
        ///   - `str_ends_with`
        ///   - `str_is_empty` (only on `A`)
        ///   - `str_is_not_empty` (only on `A`)
        operation: String,
        /// Value A to compare.
        a: String,
        /// Value B to compare. Defaults to empty string (useful for operation only on `A`).
        #[serde(default)]
        b: String,
        /// Step index to go to in the list of actions (starts at 0) if condition is true.
        step_true: String,
        /// Step index to go to in the list of actions (starts at 0) if condition is false.
        step_false: String,
    },
    /// Spawn a system command. Returns the result of the command.
    Spawn {
        /// Command to execute.
        command: String,
        /// Arguments to pass to the command. Default is empty list.
        #[serde(default)]
        args: Vec<String>,
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
    ) -> anyhow::Result<ShortcutResult> {
        match self {
            Action::Debug => CoreAction::debug(config, input_str, variables),
            Action::SetVariable { name, value } => {
                CoreAction::set_variable(input_str, variables, name, value)
            }
            Action::IncrementVariable { name,amount } => {
                CoreAction::increment_variable(input_str, variables, name,amount)
            }
            Action::DeleteVariable { name } => {
                CoreAction::delete_variable(input_str, variables, name)
            }
            Action::Sleep { duration_ms } => BasicAction::sleep(input_str, variables, duration_ms),
            Action::EndProgram => CoreAction::end_program(input_str),
            Action::GoToStep { step } => CoreAction::go_to_step(input_str, variables, step),
            Action::GoToStepRelative { step } => {
                CoreAction::go_to_step_relative(input_str, variables, step)
            }
            Action::IfElse {
                operation,
                a,
                b,
                step_true,
                step_false,
            } => CoreAction::if_else(input_str, variables, operation, a, b, step_true, step_false),
            Action::IfElseRelative {
                operation,
                a,
                b,
                step_true,
                step_false,
            } => CoreAction::if_else_relative(input_str, variables, operation, a, b, step_true, step_false),
            Action::Spawn { command, args } => {
                CoreAction::spawn(input_str, variables, command, args)
            }

            Action::PrintConsole { content } => {
                BasicAction::print_console(input_str, variables, content)
            }
            Action::ShowDialog { title, body } => {
                BasicAction::show_dialog(input_str, variables, title, body)
            }

            Action::ReadClipboard => ClipboardAction::get_clipboard_content(),
            Action::WriteClipboard { content } => {
                ClipboardAction::set_clipboard_content(input_str, variables, content)
            }

            Action::AskChatGPT { pre_prompt, prompt } => {
                OpenAIAction::ask_chat_gpt(config, input_str, variables, pre_prompt, prompt)
            }
        }
    }
}

fn action_result_str() -> String {
    "Action Result".to_string()
}

fn input_tag_str() -> String {
    "{{input}}".to_string()
}

fn one_str() -> String {
    "1".to_string()
}

pub fn replace_variables_tag(
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

pub fn replace_variables_tag_vec(
    vec_of_str: &Vec<String>,
    input_str: &str,
    variables: &HashMap<String, String>,
) -> Vec<String> {
    vec_of_str
        .iter()
        .map(|str| replace_variables_tag(str, input_str, variables))
        .collect()
}
