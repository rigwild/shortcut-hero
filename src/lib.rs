use std::ops::Deref;
use std::sync::Arc;

use copypasta::{ClipboardContext, ClipboardProvider};

pub use crate::config::Config;
use crate::hotkey::Input;

pub mod actions;
pub mod config;
pub mod hotkey;

pub fn run(config: &Config) {
    // // Uncomment to get the JSON corresponding to a shortcut
    // let shortcut = Shortcut::new(
    //     HashSet::from([
    //         KeyboardKey(KeybdKey::LControlKey),
    //         KeyboardKey(KeybdKey::BKey),
    //     ]),
    //     Action::OpenAIAskChatGPT {
    //         pre_prompt: "You should provide an explanation for the following text:\n\n".to_string(),
    //     },
    //     Input::Clipboard,
    //     Output::MessageDialog,
    // );
    // println!("shortcut: {:#?}", serde_json::to_string(&shortcut));

    register_hotkeys(config);
}

fn register_hotkeys(config: &Config) {
    let config = Arc::new(config.clone());

    config
        .clone()
        .keyboard_shortcuts
        .clone()
        .into_iter()
        .for_each(|shortcut| match shortcut.clone().key.len() {
            0 => println!("Shortcut does not have keyboard keys defined - {shortcut:#?}"),
            _ => {
                let config = config.clone();
                let first = shortcut.key.iter().nth(0).unwrap();
                first.0.bind(move || {
                    let all_pressed = shortcut.key.iter().skip(1).all(|x| x.0.is_pressed());
                    if all_pressed {
                        let input_str = get_input(&shortcut.input);
                        println!(
                            "\nRunning {:?} with input {:?} [\"{}\"]",
                            shortcut, shortcut.input, input_str
                        );
                        let result = shortcut.run(config.deref(), &input_str);
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

fn get_input(input: &Input) -> String {
    match input {
        Input::Nothing => "".to_string(),
        Input::Clipboard => get_clipboard_content(),
    }
}

fn get_clipboard_content() -> String {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.get_contents().unwrap_or_else(|_| "".to_string())
}
