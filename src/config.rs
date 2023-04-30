use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::{env, fs};

use anyhow::{anyhow, Context};
use inputbot::KeybdKey;
use serde::{Deserialize, Serialize};

use crate::actions::Action;
use crate::hotkey::{KeyboardKey, Shortcut};

const CONFIG_FILE_NAME: &'static str = "shortcut-hero.json";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(default)]
    pub openai_api_key: String,

    #[serde(default)]
    pub keyboard_shortcuts: Vec<Shortcut>,
}

impl Config {
    pub fn load_config() -> Config {
        let config = load_config_from_file().unwrap();
        println!("Config: {config:#?}\n\n");
        assert_config_valid(&config);
        config
    }

    pub fn save_config(config: &Config) {
        save_config_to_file(config).unwrap();
    }
}

fn load_config_from_file() -> anyhow::Result<Config> {
    let config_path = get_config_path();
    if !config_path.exists() {
        init_config_file();
        return Err(anyhow!(
            "Config file not found, created an empty config file at {}",
            CONFIG_FILE_NAME,
        ));
    }
    let data = fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read config file at {config_path:?}"))?;
    let config: Config = serde_json::from_str(data.as_str())?;
    Ok(config)
}

fn save_config_to_file(config: &Config) -> anyhow::Result<()> {
    let config_path = get_config_path();

    let mut file = File::create(&config_path)?;
    let json = serde_json::to_string_pretty(config)?;

    file.write_all(json.as_bytes())
        .with_context(|| format!("Could not save config to {config_path:?}"))
}

fn assert_config_valid(config: &Config) {
    config.keyboard_shortcuts.iter().for_each(|shortcut| {
        shortcut.actions.iter().for_each(|action| match action {
            Action::OpenAIAskChatGPT { .. } => {
                if config.openai_api_key.trim().is_empty()
                    || config.openai_api_key.trim() == "sk-..."
                {
                    panic!("OpenAI API key is empty or not set in configuration file")
                }
            }
            _ => {}
        })
    })
}

fn init_config_file() {
    save_config_to_file(&Config {
        openai_api_key: "sk-...".to_string(),
        keyboard_shortcuts: vec![Shortcut::new(
            std::collections::HashSet::from([
                KeyboardKey(KeybdKey::LControlKey),
                KeyboardKey(KeybdKey::BKey),
            ]),
            vec![
                Action::ReadClipboard,
                Action::Debug,
                Action::ShowDialog {
                    title: "Hello World!".to_string(),
                },
            ],
        )],
    })
    .unwrap();
}

fn get_config_path() -> PathBuf {
    env::current_dir().unwrap().join(CONFIG_FILE_NAME)
}
