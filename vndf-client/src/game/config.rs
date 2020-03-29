use std::{
    io::{
        self,
        prelude::*,
    },
    fmt,
    fs::File,
    path::Path,
};

use serde::{
    Deserialize,
    Deserializer,
    Serialize,
    Serializer,
    de::{
        self,
        Visitor,
    },
};
use winit::event::{
    MouseButton,
    VirtualKeyCode,
};

use vndf_macros::keys;


#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Config {
    pub input:       Input,
    pub color:       Color,
    pub diagnostics: Diagnostics,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Input {
    pub left:   Key,
    pub right:  Key,
    pub thrust: Key,
    pub launch: Key,
    pub quit:   Key,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Diagnostics {
    pub frame_time: bool,
    pub components: bool,
    pub input:      bool,
}

impl Config {
    pub fn load() -> Result<Self, Error> {
        let path = Path::new("vndf-config.toml");

        if path.exists() {
            let mut s = String::new();
            File::open(path)?
                .read_to_string(&mut s)?;

            let config = toml::from_str(&s)?;

            Ok(config)
        }
        else {
            let config = Self::default();

            let s = toml::to_string(&config)?;
            File::create(path)?
                .write_all(s.as_bytes())?;

            Ok(config)
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            input: Input {
                left:   Key::Keyboard(VirtualKeyCode::A),
                right:  Key::Keyboard(VirtualKeyCode::D),
                thrust: Key::Keyboard(VirtualKeyCode::W),
                launch: Key::Mouse(MouseButton::Left),
                quit:   Key::Keyboard(VirtualKeyCode::Escape),
            },
            color: Color {
                r: 1.0,
                g: 1.0,
                b: 0.0,
            },
            diagnostics: Diagnostics {
                frame_time: true,
                components: true,
                input:      true,
            },
        }
    }
}


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Key {
    Keyboard(VirtualKeyCode),
    Mouse(MouseButton),
}


keys!(
    "Key1",             Keyboard, Key1;
    "Key2",             Keyboard, Key2;
    "Key3",             Keyboard, Key3;
    "Key4",             Keyboard, Key4;
    "Key5",             Keyboard, Key5;
    "Key6",             Keyboard, Key6;
    "Key7",             Keyboard, Key7;
    "Key8",             Keyboard, Key8;
    "Key9",             Keyboard, Key9;
    "Key0",             Keyboard, Key0;
    "A",                Keyboard, A;
    "B",                Keyboard, B;
    "C",                Keyboard, C;
    "D",                Keyboard, D;
    "E",                Keyboard, E;
    "F",                Keyboard, F;
    "G",                Keyboard, G;
    "H",                Keyboard, H;
    "I",                Keyboard, I;
    "J",                Keyboard, J;
    "K",                Keyboard, K;
    "L",                Keyboard, L;
    "M",                Keyboard, M;
    "N",                Keyboard, N;
    "O",                Keyboard, O;
    "P",                Keyboard, P;
    "Q",                Keyboard, Q;
    "R",                Keyboard, R;
    "S",                Keyboard, S;
    "T",                Keyboard, T;
    "U",                Keyboard, U;
    "V",                Keyboard, V;
    "W",                Keyboard, W;
    "X",                Keyboard, X;
    "Y",                Keyboard, Y;
    "Z",                Keyboard, Z;
    "Esc",              Keyboard, Escape;
    "F1",               Keyboard, F1;
    "F2",               Keyboard, F2;
    "F3",               Keyboard, F3;
    "F4",               Keyboard, F4;
    "F5",               Keyboard, F5;
    "F6",               Keyboard, F6;
    "F7",               Keyboard, F7;
    "F8",               Keyboard, F8;
    "F9",               Keyboard, F9;
    "F10",              Keyboard, F10;
    "F11",              Keyboard, F11;
    "F12",              Keyboard, F12;
    "F13",              Keyboard, F13;
    "F14",              Keyboard, F14;
    "F15",              Keyboard, F15;
    "F16",              Keyboard, F16;
    "F17",              Keyboard, F17;
    "F18",              Keyboard, F18;
    "F19",              Keyboard, F19;
    "F20",              Keyboard, F20;
    "F21",              Keyboard, F21;
    "F22",              Keyboard, F22;
    "F23",              Keyboard, F23;
    "F24",              Keyboard, F24;
    "Snapshot",         Keyboard, Snapshot;
    "Scroll",           Keyboard, Scroll;
    "Pause",            Keyboard, Pause;
    "Insert",           Keyboard, Insert;
    "Home",             Keyboard, Home;
    "Delete",           Keyboard, Delete;
    "End",              Keyboard, End;
    "PageDown",         Keyboard, PageDown;
    "PageUp",           Keyboard, PageUp;
    "Left",             Keyboard, Left;
    "Up",               Keyboard, Up;
    "Right",            Keyboard, Right;
    "Down",             Keyboard, Down;
    "Back",             Keyboard, Back;
    "Return",           Keyboard, Return;
    "Space",            Keyboard, Space;
    "Compose",          Keyboard, Compose;
    "Caret",            Keyboard, Caret;
    "Numlock",          Keyboard, Numlock;
    "Numpad0",          Keyboard, Numpad0;
    "Numpad1",          Keyboard, Numpad1;
    "Numpad2",          Keyboard, Numpad2;
    "Numpad3",          Keyboard, Numpad3;
    "Numpad4",          Keyboard, Numpad4;
    "Numpad5",          Keyboard, Numpad5;
    "Numpad6",          Keyboard, Numpad6;
    "Numpad7",          Keyboard, Numpad7;
    "Numpad8",          Keyboard, Numpad8;
    "Numpad9",          Keyboard, Numpad9;
    "AbntC1",           Keyboard, AbntC1;
    "AbntC2",           Keyboard, AbntC2;
    "Add",              Keyboard, Add;
    "Apostrophe",       Keyboard, Apostrophe;
    "Apps",             Keyboard, Apps;
    "At",               Keyboard, At;
    "Ax",               Keyboard, Ax;
    "Backslash",        Keyboard, Backslash;
    "Calculator",       Keyboard, Calculator;
    "Capital",          Keyboard, Capital;
    "Colon",            Keyboard, Colon;
    "Comma",            Keyboard, Comma;
    "Convert",          Keyboard, Convert;
    "Decimal",          Keyboard, Decimal;
    "Divide",           Keyboard, Divide;
    "Equals",           Keyboard, Equals;
    "Grave",            Keyboard, Grave;
    "Kana",             Keyboard, Kana;
    "Kanji",            Keyboard, Kanji;
    "LAlt",             Keyboard, LAlt;
    "LBracket",         Keyboard, LBracket;
    "LControl",         Keyboard, LControl;
    "LShift",           Keyboard, LShift;
    "LWin",             Keyboard, LWin;
    "Mail",             Keyboard, Mail;
    "MediaSelect",      Keyboard, MediaSelect;
    "MediaStop",        Keyboard, MediaStop;
    "Minus",            Keyboard, Minus;
    "Multiply",         Keyboard, Multiply;
    "Mute",             Keyboard, Mute;
    "MyComputer",       Keyboard, MyComputer;
    "NavigateForward",  Keyboard, NavigateForward;
    "NavigateBackward", Keyboard, NavigateBackward;
    "NextTrack",        Keyboard, NextTrack;
    "NoConvert",        Keyboard, NoConvert;
    "NumpadComma",      Keyboard, NumpadComma;
    "NumpadEnter",      Keyboard, NumpadEnter;
    "NumpadEquals",     Keyboard, NumpadEquals;
    "OEM102",           Keyboard, OEM102;
    "Period",           Keyboard, Period;
    "PlayPause",        Keyboard, PlayPause;
    "Power",            Keyboard, Power;
    "PrevTrack",        Keyboard, PrevTrack;
    "RAlt",             Keyboard, RAlt;
    "RBracket",         Keyboard, RBracket;
    "RControl",         Keyboard, RControl;
    "RShift",           Keyboard, RShift;
    "RWin",             Keyboard, RWin;
    "Semicolon",        Keyboard, Semicolon;
    "Slash",            Keyboard, Slash;
    "Sleep",            Keyboard, Sleep;
    "Stop",             Keyboard, Stop;
    "Subtract",         Keyboard, Subtract;
    "Sysrq",            Keyboard, Sysrq;
    "Tab",              Keyboard, Tab;
    "Underline",        Keyboard, Underline;
    "Unlabeled",        Keyboard, Unlabeled;
    "VolumeDown",       Keyboard, VolumeDown;
    "VolumeUp",         Keyboard, VolumeUp;
    "Wake",             Keyboard, Wake;
    "WebBack",          Keyboard, WebBack;
    "WebFavorites",     Keyboard, WebFavorites;
    "WebForward",       Keyboard, WebForward;
    "WebHome",          Keyboard, WebHome;
    "WebRefresh",       Keyboard, WebRefresh;
    "WebSearch",        Keyboard, WebSearch;
    "WebStop",          Keyboard, WebStop;
    "Yen",              Keyboard, Yen;
    "Copy",             Keyboard, Copy;
    "Paste",            Keyboard, Paste;
    "Cut",              Keyboard, Cut;

    "MouseLeft",   Mouse, Left;
    "MouseRight",  Mouse, Right;
    "MouseMiddle", Mouse, Middle;
);


#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Load(toml::de::Error),
    Store(toml::ser::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Self::Load(err)
    }
}

impl From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Self {
        Self::Store(err)
    }
}
