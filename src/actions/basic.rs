use std::process::Command;

use anyhow::anyhow;
use native_dialog::{MessageDialog, MessageType};

pub struct BasicAction;

impl BasicAction {
    pub fn print_console(input_str: &str) -> anyhow::Result<String> {
        println!("{input_str}");
        Ok(input_str.to_string())
    }

    pub fn show_dialog(title: &str, content: &str) -> anyhow::Result<String> {
        MessageDialog::new()
            .set_type(MessageType::Info)
            .set_title(title)
            .set_text(content)
            .show_alert()
            .unwrap();
        Ok(content.to_string())
    }

    pub fn spawn(command: &str, args: &Vec<String>, last_arg: &str) -> anyhow::Result<String> {
        let mut command = Command::new(command);
        args.iter().for_each(|arg| {
            command.arg(arg);
        });
        command.arg(last_arg);
        let command = command.output().expect("Failed to execute command");
        if command.status.success() {
            Ok(String::from_utf8_lossy(&command.stdout).to_string())
        } else {
            Err(anyhow!(
                "Command failed: {}",
                String::from_utf8_lossy(&command.stderr)
            ))
        }
    }
}
