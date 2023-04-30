use std::ops::Deref;
use std::sync::Arc;

use inputbot::KeybdKey;

pub use crate::config::Config;
use crate::hotkey::{KeyboardKey, Shortcut};

pub mod actions;
pub mod config;
pub mod hotkey;

pub fn run(config: &Config) {
    // // Uncomment to get the JSON corresponding to some shortcut
    // let shortcut = Shortcut::new(
    //     vec![
    //         KeyboardKey(KeybdKey::LControlKey),
    //         KeyboardKey(KeybdKey::BKey),
    //     ],
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
                let mut keys = shortcut.clone().keys;
                sort_keys_with_modifier_last(&mut keys);
                let keys = keys.into_iter();

                let first = keys.clone().nth(0).unwrap();
                let rest = keys.clone().skip(1);
                first.0.bind(move || {
                    let all_pressed = rest.clone().all(|x| x.0.is_pressed());
                    if all_pressed {
                        run_shortcut(&shortcut, &config)
                    }
                });
            }
        });

    inputbot::handle_input_events();
}

/// Put the modifiers at the end of the vector, else the keys combo might not work
fn sort_keys_with_modifier_last(keys: &mut Vec<KeyboardKey>) {
    let modifiers = [
        KeybdKey::LSuper,
        KeybdKey::RSuper,
        KeybdKey::LShiftKey,
        KeybdKey::RShiftKey,
        KeybdKey::LControlKey,
        KeybdKey::RControlKey,
        KeybdKey::LAltKey,
        KeybdKey::RAltKey,
    ];

    keys.sort_by(|a, b| {
        let a_is_modifier = modifiers.iter().find(|x| x == &&a.0).is_some();
        let b_is_modifier = modifiers.iter().find(|x| x == &&b.0).is_some();

        match (a_is_modifier, b_is_modifier) {
            (true, true) => std::cmp::Ordering::Equal,
            (true, false) => std::cmp::Ordering::Greater,
            (false, true) => std::cmp::Ordering::Less,
            (false, false) => std::cmp::Ordering::Equal,
        }
    });
}

fn run_shortcut(shortcut: &Shortcut, config: &Arc<Config>) {
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
