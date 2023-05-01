use std::process::Command;

use anyhow::anyhow;
use native_dialog::{MessageDialog, MessageType};

pub struct BasicAction;

impl BasicAction {
    pub fn sleep(duration_ms: &str) -> anyhow::Result<()> {
        let duration_ms = duration_ms.parse::<u64>().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(duration_ms));
        Ok(())
    }

    pub fn print_console(content: &str) -> anyhow::Result<()> {
        println!("{content}");
        Ok(())
    }

    pub fn show_dialog(title: &str, content: &str) -> anyhow::Result<()> {
        MessageDialog::new()
            .set_type(MessageType::Info)
            .set_title(title)
            .set_text(content)
            .show_alert()
            .unwrap();
        Ok(())
    }

    pub fn spawn(command: &str, args: &Vec<String>) -> anyhow::Result<String> {
        let mut command = Command::new(command);
        args.iter().for_each(|arg| {
            command.arg(arg);
        });
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
