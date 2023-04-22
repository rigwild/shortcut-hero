pub mod actions;
pub mod config;

use crate::actions::Action;
pub use crate::config::Config;

pub fn run(config: &Config) -> anyhow::Result<()> {
    let some_action = Action::AskChatGPT {
        pre_prompt: "".to_string(),
    };
    let res = some_action.run(config, "who created you?")?;
    println!("Reponse from action: {res}");
    Ok(())
}
