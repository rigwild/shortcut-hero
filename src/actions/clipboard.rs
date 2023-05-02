use std::collections::HashMap;

use copypasta::{ClipboardContext, ClipboardProvider};

use crate::evaluation::replace_variables_tag;
use crate::hotkey::ShortcutResult;

pub struct ClipboardAction;

impl ClipboardAction {
    pub fn get_clipboard_content() -> anyhow::Result<ShortcutResult> {
        let mut ctx = ClipboardContext::new().unwrap();
        Ok(ShortcutResult::Success(
            ctx.get_contents().unwrap_or_else(|_| "".to_string()),
        ))
    }

    pub fn set_clipboard_content(
        input_str: &str,
        variables: &HashMap<String, String>,
        content: &str,
    ) -> anyhow::Result<ShortcutResult> {
        let content = replace_variables_tag(content, input_str, variables);
        let mut ctx = ClipboardContext::new().unwrap();
        ctx.set_contents(content.to_owned())
            .expect("Failed to save content to clipboard");
        Ok(ShortcutResult::Success(input_str.to_string()))
    }
}
