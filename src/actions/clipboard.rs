use copypasta::{ClipboardContext, ClipboardProvider};

pub struct ClipboardAction;

impl ClipboardAction {
    pub fn get_clipboard_content() -> anyhow::Result<String> {
        let mut ctx = ClipboardContext::new().unwrap();
        Ok(ctx.get_contents().unwrap_or_else(|_| "".to_string()))
    }

    pub fn set_clipboard_content(content: &str) -> anyhow::Result<()> {
        let mut ctx = ClipboardContext::new().unwrap();
        ctx.set_contents(content.to_owned())
            .expect("Failed to save content to clipboard");
        Ok(())
    }
}
