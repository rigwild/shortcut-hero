use crate::actions::Action;
use crate::Config;
use inputbot::KeybdKey;
use native_dialog::{MessageDialog, MessageType};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt::Debug;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shortcut {
    pub key: HashSet<KeyboardKey>,
    pub action: Action,
    #[serde(default)]
    pub output: Output,
}

impl Shortcut {
    pub fn new(key: HashSet<KeyboardKey>, action: Action, output: Output) -> Self {
        Shortcut {
            key,
            action,
            output,
        }
    }

    pub fn with_output_dialog(key: HashSet<KeyboardKey>, action: Action) -> Self {
        Shortcut::new(key, action, Output::MessageDialog)
    }

    pub fn run(&self, config: &Config, input_str: &str) -> anyhow::Result<String> {
        let action_result = self.action.run(config, input_str)?;
        match self.output {
            Output::MessageDialog => {
                show_dialog(&format!("Result of action {}", self.action), &action_result);
            }
        }
        Ok(action_result)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "name", rename_all = "snake_case")]
pub enum Output {
    #[serde(rename = "dialog")]
    MessageDialog,
}

impl Default for Output {
    fn default() -> Self {
        Output::MessageDialog
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
