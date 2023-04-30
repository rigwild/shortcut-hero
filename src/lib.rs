use std::ops::Deref;
use std::sync::Arc;

pub use crate::config::Config;

pub mod actions;
pub mod config;
pub mod hotkey;

pub fn run(config: &Config) -> anyhow::Result<()> {
    let input_str = "Whats your name?";

    let config_arc = Arc::new(config.clone());
    let config = Arc::clone(&config_arc);

    config
        .keyboard_shortcuts
        .clone()
        .into_iter()
        .for_each(|shortcut| match shortcut.clone().key.len() {
            0 => {
                println!(
                    "Shortcut does not have keyboard keys defined - {:#?}",
                    shortcut
                )
            }
            _ => {
                let config = config.clone();
                let first = shortcut.key.iter().nth(1).unwrap();
                first.0.bind(move || {
                    let all_pressed = shortcut.key.iter().skip(1).all(|x| x.0.is_pressed());
                    if all_pressed {
                        let result = shortcut.run(config.deref(), input_str);
                        match result {
                            Ok(result_str) => {
                                // Random idea: use the result from the action and chain actions?
                                println!("Result from {:?}: {}", shortcut, result_str)
                            }
                            Err(e) => {
                                eprintln!("Shortcut {:?} failed: {}", shortcut, e)
                            }
                        }
                    }
                });
            }
        });

    inputbot::handle_input_events();

    // let some_action = Action::OpenAIAskChatGPT {
    //     pre_prompt: "".to_string(),
    // };
    // let shortcut = Shortcut::with_output_dialog(
    //     HashSet::from([
    //         KeyboardKey(KeybdKey::LControlKey),
    //         KeyboardKey(KeybdKey::BKey),
    //     ]),
    //     Action::OpenAIAskChatGPT {
    //         pre_prompt: "pre-prompt content string".to_string(),
    //     },
    // );
    // println!("shortcut: {:#?}", serde_json::to_string(&shortcut));

    Ok(())
}
