use std::collections::HashMap;
use std::fmt::Debug;

use anyhow::anyhow;
pub use inputbot::KeybdKey;
use rand::Rng;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::actions::Action;
use crate::Config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shortcut {
    #[serde(default)]
    pub description: String,
    pub keys: Vec<KeyboardKey>,
    pub actions: Vec<Action>,
}

impl Shortcut {
    pub fn new(description: &str, keys: Vec<KeyboardKey>, actions: Vec<Action>) -> Self {
        Shortcut {
            description: description.to_string(),
            keys,
            actions,
        }
    }

    pub fn new_no_description(keys: Vec<KeyboardKey>, actions: Vec<Action>) -> Self {
        Shortcut {
            description: "".to_string(),
            keys,
            actions,
        }
    }

    pub fn trigger(&self, config: &Config) -> anyhow::Result<Vec<String>> {
        let trigger_id: u32 = rand::thread_rng().gen();

        let mut variables: HashMap<String, String> = HashMap::new();
        let mut input_str = "".to_string();

        let mut full_actions_result: Vec<String> = Vec::new();
        let mut i_action = 0;
        while i_action < self.actions.len() {
            let mut used_go_to = false;

            let action = &self.actions[i_action];
            let shortcut_result = action.run(config, &input_str, &mut variables)?;

            match shortcut_result {
                ShortcutResult::Success(output) => {
                    input_str = output.clone();
                }

                ShortcutResult::GoToStep { output, step } => {
                    input_str = output.clone();
                    if step >= self.actions.len() {
                        return Err(anyhow!(
                            "Step to go to [from {} to {}] is out of bounds [0, {}] )",
                            i_action,
                            step,
                            self.actions.len() - 1
                        ))?;
                    }
                    i_action = step;
                    used_go_to = true;
                }

                ShortcutResult::GoToStepRelative {
                    output,
                    step,
                    sign_is_positive,
                } => {
                    input_str = output.clone();
                    let step_absolute: usize = step as usize;
                    if !sign_is_positive {
                        if step_absolute > i_action {
                            return Err(anyhow!(
                                "Step to go to [from {} to {} (-{})] is out of bounds [0, {}] )",
                                i_action,
                                i_action - step_absolute,
                                step,
                                self.actions.len() - 1
                            ))?;
                        } else {
                            i_action = i_action - step_absolute;
                            used_go_to = true;
                        }
                    } else {
                        if i_action + step_absolute >= self.actions.len() {
                            return Err(anyhow!(
                                "Step to go to [from {} to {} (+{})] is out of bounds [0, {}] )",
                                i_action,
                                i_action + step_absolute,
                                step,
                                self.actions.len() - 1
                            ))?;
                        } else {
                            i_action = i_action + step_absolute;
                            used_go_to = true;
                        }
                    }
                }

                ShortcutResult::EndProgram(output) => {
                    input_str = output.clone();
                    i_action = usize::MAX;
                    used_go_to = true;
                }
            }
            let action_result = format!("{:?}, Output: {}", action, input_str);
            full_actions_result.push(action_result.clone());
            println!("[Action Run ID={trigger_id}] {action_result}");

            // Do not increment if one of the shortcuts changed the action index
            if !used_go_to {
                i_action += 1;
            }
        }
        Ok(full_actions_result)
    }
}

/// The result of a shortcut action
///
/// Every result should contain at least the output string
pub enum ShortcutResult {
    Success(String),
    GoToStep {
        output: String,
        step: usize,
    },
    GoToStepRelative {
        output: String,
        step: usize,
        sign_is_positive: bool,
    },
    EndProgram(String),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct KeyboardKey(#[serde(with = "KeybdKeyDef")] pub KeybdKey);

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Serialize, Deserialize)]
#[serde(remote = "KeybdKey")]
#[derive(TS)]
#[ts(export = "enum")]
enum KeybdKeyDef {
    BackspaceKey,
    TabKey,
    EnterKey,
    EscapeKey,
    SpaceKey,
    PageUpKey,
    PageDownKey,
    EndKey,
    HomeKey,
    LeftKey,
    UpKey,
    RightKey,
    DownKey,
    InsertKey,
    DeleteKey,
    Numrow0Key,
    Numrow1Key,
    Numrow2Key,
    Numrow3Key,
    Numrow4Key,
    Numrow5Key,
    Numrow6Key,
    Numrow7Key,
    Numrow8Key,
    Numrow9Key,
    AKey,
    BKey,
    CKey,
    DKey,
    EKey,
    FKey,
    GKey,
    HKey,
    IKey,
    JKey,
    KKey,
    LKey,
    MKey,
    NKey,
    OKey,
    PKey,
    QKey,
    RKey,
    SKey,
    TKey,
    UKey,
    VKey,
    WKey,
    XKey,
    YKey,
    ZKey,
    LSuper,
    RSuper,
    Numpad0Key,
    Numpad1Key,
    Numpad2Key,
    Numpad3Key,
    Numpad4Key,
    Numpad5Key,
    Numpad6Key,
    Numpad7Key,
    Numpad8Key,
    Numpad9Key,
    F1Key,
    F2Key,
    F3Key,
    F4Key,
    F5Key,
    F6Key,
    F7Key,
    F8Key,
    F9Key,
    F10Key,
    F11Key,
    F12Key,
    F13Key,
    F14Key,
    F15Key,
    F16Key,
    F17Key,
    F18Key,
    F19Key,
    F20Key,
    F21Key,
    F22Key,
    F23Key,
    F24Key,
    NumLockKey,
    ScrollLockKey,
    CapsLockKey,
    LShiftKey,
    RShiftKey,
    LControlKey,
    RControlKey,
    LAltKey,
    RAltKey,

    BrowserBackKey,
    BrowserForwardKey,
    BrowserRefreshKey,

    VolumeMuteKey,
    VolumeDownKey,
    VolumeUpKey,

    MediaNextTrackKey,
    MediaPrevTrackKey,
    MediaStopKey,
    MediaPlayPauseKey,

    BackquoteKey,
    SlashKey,
    BackslashKey,
    CommaKey,
    PeriodKey,
    MinusKey,
    QuoteKey,
    SemicolonKey,
    LBracketKey,
    RBracketKey,
    EqualKey,

    #[ts(skip)]
    OtherKey(u64),
}
