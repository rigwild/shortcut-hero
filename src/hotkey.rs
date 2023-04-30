use std::collections::HashSet;
use std::fmt::{Debug};

use inputbot::KeybdKey;
use native_dialog::{MessageDialog, MessageType};
use serde::{Deserialize, Serialize};

use crate::actions::Action;
use crate::Config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shortcut {
    pub key: HashSet<KeyboardKey>,
    pub actions: Vec<Action>,
    #[serde(default)]
    pub input: Input,
    #[serde(default)]
    pub output: Output,
}

/// Same as `Shortcut`, but made for chained actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortCutSub {
    pub action: Action,
    #[serde(default)]
    pub input: Input,
    #[serde(default)]
    pub output: Output,
}

impl Shortcut {
    pub fn new(
        key: HashSet<KeyboardKey>,
        actions: Vec<Action>,
        input: Input,
        output: Output,
    ) -> Self {
        Shortcut {
            key,
            actions,
            input,
            output,
        }
    }

    pub fn run(&self, config: &Config, input_str: &str) -> anyhow::Result<String> {
        let mut previous_action_result = input_str.to_string();
        let mut full_actions_result: Vec<String> = Vec::new();
        for action in &self.actions {
            let result = action.run(config, &previous_action_result)?;
            previous_action_result = result.clone();
            full_actions_result.push(format!("[{} - {}]", action, result.clone()));
        }
        match self.output {
            Output::Console => println!(
                "Result of actions {:#?} is {}",
                self.actions,
                full_actions_result.join(", ")
            ),
            Output::MessageDialog => show_dialog(
                &format!("Result of actions {:#?}", self.actions),
                &full_actions_result.join(", "),
            ),
        }
        Ok(previous_action_result)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Input {
    Nothing,
    Clipboard,
}

impl Default for Input {
    fn default() -> Self {
        Input::Nothing
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Output {
    Console,
    #[serde(rename = "dialog")]
    MessageDialog,
}

impl Default for Output {
    fn default() -> Self {
        Output::Console
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct KeyboardKey(#[serde(with = "KeybdKeyDef")] pub KeybdKey);

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Serialize, Deserialize)]
#[serde(remote = "KeybdKey")]
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

    OtherKey(u64),
}

fn show_dialog(title: &str, text: &str) {
    MessageDialog::new()
        .set_type(MessageType::Info)
        .set_title(title)
        .set_text(&format!("{:#?}", text))
        .show_alert()
        .unwrap();
}
