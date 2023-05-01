use std::collections::HashMap;
use std::fmt::Debug;

pub use inputbot::KeybdKey;
use serde::{Deserialize, Serialize};

use crate::actions::Action;
use crate::Config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shortcut {
    pub keys: Vec<KeyboardKey>,
    pub actions: Vec<Action>,
}

impl Shortcut {
    pub fn new(keys: Vec<KeyboardKey>, actions: Vec<Action>) -> Self {
        Shortcut { keys, actions }
    }

    pub fn trigger(&self, config: &Config) -> anyhow::Result<Vec<String>> {
        let mut variables: HashMap<String, String> = HashMap::new();
        let mut input_str = "".to_string();

        let mut full_actions_result: Vec<String> = Vec::new();
        for action in &self.actions {
            let result = action.run(config, &input_str, &mut variables)?;
            input_str = result.clone();
            full_actions_result.push(format!("[{:?} - {}]", action, result.clone()));
        }
        Ok(full_actions_result)
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
