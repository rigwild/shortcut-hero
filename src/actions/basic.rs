use std::collections::HashMap;
use std::process::Command;

use anyhow::anyhow;
use native_dialog::{MessageDialog, MessageType};

use crate::actions::{replace_variables_tag, replace_variables_tag_vec};
use crate::hotkey::ShortcutResult;

pub struct BasicAction;

impl BasicAction {
    pub fn sleep(
        input_str: &str,
        variables: &HashMap<String, String>,
        duration_ms: &str,
    ) -> anyhow::Result<ShortcutResult> {
        let duration_ms = replace_variables_tag(duration_ms, input_str, variables);
        let duration_ms = duration_ms.parse::<u64>().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(duration_ms));
        Ok(ShortcutResult::Success(input_str.to_string()))
    }

    pub fn print_console(
        input_str: &str,
        variables: &HashMap<String, String>,
        content: &str,
    ) -> anyhow::Result<ShortcutResult> {
        let content = replace_variables_tag(content, input_str, variables);

        println!("{content}");
        Ok(ShortcutResult::Success(input_str.to_string()))
    }

    pub fn show_dialog(
        input_str: &str,
        variables: &HashMap<String, String>,
        title: &str,
        body: &str,
    ) -> anyhow::Result<ShortcutResult> {
        let title = replace_variables_tag(title, input_str, variables);
        let body = replace_variables_tag(body, input_str, variables);
        MessageDialog::new()
            .set_type(MessageType::Info)
            .set_title(&title)
            .set_text(&body)
            .show_alert()
            .unwrap();
        Ok(ShortcutResult::Success(input_str.to_string()))
    }

    pub fn spawn(
        input_str: &str,
        variables: &HashMap<String, String>,
        command: &str,
        args: &Vec<String>,
    ) -> anyhow::Result<ShortcutResult> {
        let command = replace_variables_tag(command, input_str, variables);
        let args = replace_variables_tag_vec(args, input_str, variables);

        let mut command = Command::new(command);
        args.iter().for_each(|arg| {
            command.arg(arg);
        });
        let command = command.output().expect("Failed to execute command");
        if command.status.success() {
            Ok(ShortcutResult::Success(
                String::from_utf8_lossy(&command.stdout).to_string(),
            ))
        } else {
            Err(anyhow!(
                "Command failed: {}",
                String::from_utf8_lossy(&command.stderr)
            ))
        }
    }
}
