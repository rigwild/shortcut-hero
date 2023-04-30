use std::ops::Deref;
use std::sync::Arc;

pub use crate::config::Config;

pub mod actions;
pub mod config;
pub mod hotkey;

pub fn run(config: &Config) {
    // Uncomment to get the JSON corresponding to a shortcut
    // let shortcut = hotkey::Shortcut::new(
    //     std::collections::HashSet::from([
    //         hotkey::KeyboardKey(hotkey::KeybdKey::LControlKey),
    //         hotkey::KeyboardKey(hotkey::KeybdKey::BKey),
    //     ]),
    //     vec![
    //         actions::Action::ReadClipboard,
    //         actions::Action::Debug,
    //         actions::Action::OpenAIAskChatGPT {
    //             pre_prompt: "Explain to me the following text by talking like I am a 5 years old"
    //                 .to_string(),
    //         },
    //         actions::Action::ShowDialog {
    //             title: "ChatGPT Explain".to_string(),
    //         },
    //     ],
    // );
    // println!(
    //     "Shortcut JSON:\n\n{}",
    //     serde_json::to_string_pretty(&shortcut).unwrap()
    // );

    register_hotkeys(config);
}

fn register_hotkeys(config: &Config) {
    let config = Arc::new(config.clone());

    config
        .clone()
        .keyboard_shortcuts
        .clone()
        .into_iter()
        .for_each(|shortcut| match shortcut.clone().keys.len() {
            0 => println!("Shortcut does not have keyboard keys defined - {shortcut:#?}"),
            _ => {
                let config = config.clone();
                let first = shortcut.keys.iter().nth(0).unwrap();
                first.0.bind(move || {
                    let all_pressed = shortcut.keys.iter().skip(1).all(|x| x.0.is_pressed());
                    if all_pressed {
                        println!("\nRunning {:?}", shortcut);
                        let result = shortcut.trigger(config.deref());
                        match result {
                            Ok(_result_str) => {
                                // println!("Result [{}] for {:?}", result_str, shortcut)
                            }
                            Err(e) => {
                                eprintln!("Failed [{}] for {:?}", e, shortcut)
                            }
                        }
                    }
                });
            }
        });

    inputbot::handle_input_events();
}
