use std::{
    io::{
        self,
        prelude::*,
    },
    fmt,
    fs::File,
    path::Path,
};

use ggez::event::{
    KeyCode,
    MouseButton,
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
                left:   Key::Keyboard(KeyCode::A),
                right:  Key::Keyboard(KeyCode::D),
                thrust: Key::Keyboard(KeyCode::W),
                launch: Key::Mouse(MouseButton::Left),
                quit:   Key::Keyboard(KeyCode::Escape),
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
    Keyboard(KeyCode),
    Mouse(MouseButton),
}

macro_rules! keys {
    ($($s:tt, $type:ident, $type2:ident, $key:ident;)*) => {
        impl Serialize for Key {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: Serializer
            {
                #[allow(unused_parens)]
                let expr = match self {
                    $(Key::$type($type2::$key) => $s,)*

                    key => panic!("Variant not allowed: {:?}", key),
                };

                serializer.serialize_str(expr)
            }
        }

        impl<'de> Deserialize<'de> for Key {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: Deserializer<'de>
            {
                deserializer.deserialize_str(KeyVisitor)
            }
        }


        struct KeyVisitor;

        impl<'de> Visitor<'de> for KeyVisitor {
            type Value = Key;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a key identifier")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where E: de::Error,
            {
                #[allow(unused_parens)]
                match value {
                    $($s => Ok(Key::$type($type2::$key)),)*

                    _ =>
                        Err(
                            E::custom(
                                format!("not a key identifier: {}", value)
                            )
                        )
                }
            }
        }

        impl fmt::Display for Key {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                #[allow(unused_parens)]
                match self {
                    $(Key::$type($type2::$key) => write!(f, "{}", $s),)*

                    _ => write!(f, "Unknown key"),
                }
            }
        }
    };
}

keys!(
    "Key1",             Keyboard, KeyCode, Key1;
    "Key2",             Keyboard, KeyCode, Key2;
    "Key3",             Keyboard, KeyCode, Key3;
    "Key4",             Keyboard, KeyCode, Key4;
    "Key5",             Keyboard, KeyCode, Key5;
    "Key6",             Keyboard, KeyCode, Key6;
    "Key7",             Keyboard, KeyCode, Key7;
    "Key8",             Keyboard, KeyCode, Key8;
    "Key9",             Keyboard, KeyCode, Key9;
    "Key0",             Keyboard, KeyCode, Key0;
    "A",                Keyboard, KeyCode, A;
    "B",                Keyboard, KeyCode, B;
    "C",                Keyboard, KeyCode, C;
    "D",                Keyboard, KeyCode, D;
    "E",                Keyboard, KeyCode, E;
    "F",                Keyboard, KeyCode, F;
    "G",                Keyboard, KeyCode, G;
    "H",                Keyboard, KeyCode, H;
    "I",                Keyboard, KeyCode, I;
    "J",                Keyboard, KeyCode, J;
    "K",                Keyboard, KeyCode, K;
    "L",                Keyboard, KeyCode, L;
    "M",                Keyboard, KeyCode, M;
    "N",                Keyboard, KeyCode, N;
    "O",                Keyboard, KeyCode, O;
    "P",                Keyboard, KeyCode, P;
    "Q",                Keyboard, KeyCode, Q;
    "R",                Keyboard, KeyCode, R;
    "S",                Keyboard, KeyCode, S;
    "T",                Keyboard, KeyCode, T;
    "U",                Keyboard, KeyCode, U;
    "V",                Keyboard, KeyCode, V;
    "W",                Keyboard, KeyCode, W;
    "X",                Keyboard, KeyCode, X;
    "Y",                Keyboard, KeyCode, Y;
    "Z",                Keyboard, KeyCode, Z;
    "Esc",              Keyboard, KeyCode, Escape;
    "F1",               Keyboard, KeyCode, F1;
    "F2",               Keyboard, KeyCode, F2;
    "F3",               Keyboard, KeyCode, F3;
    "F4",               Keyboard, KeyCode, F4;
    "F5",               Keyboard, KeyCode, F5;
    "F6",               Keyboard, KeyCode, F6;
    "F7",               Keyboard, KeyCode, F7;
    "F8",               Keyboard, KeyCode, F8;
    "F9",               Keyboard, KeyCode, F9;
    "F10",              Keyboard, KeyCode, F10;
    "F11",              Keyboard, KeyCode, F11;
    "F12",              Keyboard, KeyCode, F12;
    "F13",              Keyboard, KeyCode, F13;
    "F14",              Keyboard, KeyCode, F14;
    "F15",              Keyboard, KeyCode, F15;
    "F16",              Keyboard, KeyCode, F16;
    "F17",              Keyboard, KeyCode, F17;
    "F18",              Keyboard, KeyCode, F18;
    "F19",              Keyboard, KeyCode, F19;
    "F20",              Keyboard, KeyCode, F20;
    "F21",              Keyboard, KeyCode, F21;
    "F22",              Keyboard, KeyCode, F22;
    "F23",              Keyboard, KeyCode, F23;
    "F24",              Keyboard, KeyCode, F24;
    "Snapshot",         Keyboard, KeyCode, Snapshot;
    "Scroll",           Keyboard, KeyCode, Scroll;
    "Pause",            Keyboard, KeyCode, Pause;
    "Insert",           Keyboard, KeyCode, Insert;
    "Home",             Keyboard, KeyCode, Home;
    "Delete",           Keyboard, KeyCode, Delete;
    "End",              Keyboard, KeyCode, End;
    "PageDown",         Keyboard, KeyCode, PageDown;
    "PageUp",           Keyboard, KeyCode, PageUp;
    "Left",             Keyboard, KeyCode, Left;
    "Up",               Keyboard, KeyCode, Up;
    "Right",            Keyboard, KeyCode, Right;
    "Down",             Keyboard, KeyCode, Down;
    "Back",             Keyboard, KeyCode, Back;
    "Return",           Keyboard, KeyCode, Return;
    "Space",            Keyboard, KeyCode, Space;
    "Compose",          Keyboard, KeyCode, Compose;
    "Caret",            Keyboard, KeyCode, Caret;
    "Numlock",          Keyboard, KeyCode, Numlock;
    "Numpad0",          Keyboard, KeyCode, Numpad0;
    "Numpad1",          Keyboard, KeyCode, Numpad1;
    "Numpad2",          Keyboard, KeyCode, Numpad2;
    "Numpad3",          Keyboard, KeyCode, Numpad3;
    "Numpad4",          Keyboard, KeyCode, Numpad4;
    "Numpad5",          Keyboard, KeyCode, Numpad5;
    "Numpad6",          Keyboard, KeyCode, Numpad6;
    "Numpad7",          Keyboard, KeyCode, Numpad7;
    "Numpad8",          Keyboard, KeyCode, Numpad8;
    "Numpad9",          Keyboard, KeyCode, Numpad9;
    "AbntC1",           Keyboard, KeyCode, AbntC1;
    "AbntC2",           Keyboard, KeyCode, AbntC2;
    "Add",              Keyboard, KeyCode, Add;
    "Apostrophe",       Keyboard, KeyCode, Apostrophe;
    "Apps",             Keyboard, KeyCode, Apps;
    "At",               Keyboard, KeyCode, At;
    "Ax",               Keyboard, KeyCode, Ax;
    "Backslash",        Keyboard, KeyCode, Backslash;
    "Calculator",       Keyboard, KeyCode, Calculator;
    "Capital",          Keyboard, KeyCode, Capital;
    "Colon",            Keyboard, KeyCode, Colon;
    "Comma",            Keyboard, KeyCode, Comma;
    "Convert",          Keyboard, KeyCode, Convert;
    "Decimal",          Keyboard, KeyCode, Decimal;
    "Divide",           Keyboard, KeyCode, Divide;
    "Equals",           Keyboard, KeyCode, Equals;
    "Grave",            Keyboard, KeyCode, Grave;
    "Kana",             Keyboard, KeyCode, Kana;
    "Kanji",            Keyboard, KeyCode, Kanji;
    "LAlt",             Keyboard, KeyCode, LAlt;
    "LBracket",         Keyboard, KeyCode, LBracket;
    "LControl",         Keyboard, KeyCode, LControl;
    "LShift",           Keyboard, KeyCode, LShift;
    "LWin",             Keyboard, KeyCode, LWin;
    "Mail",             Keyboard, KeyCode, Mail;
    "MediaSelect",      Keyboard, KeyCode, MediaSelect;
    "MediaStop",        Keyboard, KeyCode, MediaStop;
    "Minus",            Keyboard, KeyCode, Minus;
    "Multiply",         Keyboard, KeyCode, Multiply;
    "Mute",             Keyboard, KeyCode, Mute;
    "MyComputer",       Keyboard, KeyCode, MyComputer;
    "NavigateForward",  Keyboard, KeyCode, NavigateForward;
    "NavigateBackward", Keyboard, KeyCode, NavigateBackward;
    "NextTrack",        Keyboard, KeyCode, NextTrack;
    "NoConvert",        Keyboard, KeyCode, NoConvert;
    "NumpadComma",      Keyboard, KeyCode, NumpadComma;
    "NumpadEnter",      Keyboard, KeyCode, NumpadEnter;
    "NumpadEquals",     Keyboard, KeyCode, NumpadEquals;
    "OEM102",           Keyboard, KeyCode, OEM102;
    "Period",           Keyboard, KeyCode, Period;
    "PlayPause",        Keyboard, KeyCode, PlayPause;
    "Power",            Keyboard, KeyCode, Power;
    "PrevTrack",        Keyboard, KeyCode, PrevTrack;
    "RAlt",             Keyboard, KeyCode, RAlt;
    "RBracket",         Keyboard, KeyCode, RBracket;
    "RControl",         Keyboard, KeyCode, RControl;
    "RShift",           Keyboard, KeyCode, RShift;
    "RWin",             Keyboard, KeyCode, RWin;
    "Semicolon",        Keyboard, KeyCode, Semicolon;
    "Slash",            Keyboard, KeyCode, Slash;
    "Sleep",            Keyboard, KeyCode, Sleep;
    "Stop",             Keyboard, KeyCode, Stop;
    "Subtract",         Keyboard, KeyCode, Subtract;
    "Sysrq",            Keyboard, KeyCode, Sysrq;
    "Tab",              Keyboard, KeyCode, Tab;
    "Underline",        Keyboard, KeyCode, Underline;
    "Unlabeled",        Keyboard, KeyCode, Unlabeled;
    "VolumeDown",       Keyboard, KeyCode, VolumeDown;
    "VolumeUp",         Keyboard, KeyCode, VolumeUp;
    "Wake",             Keyboard, KeyCode, Wake;
    "WebBack",          Keyboard, KeyCode, WebBack;
    "WebFavorites",     Keyboard, KeyCode, WebFavorites;
    "WebForward",       Keyboard, KeyCode, WebForward;
    "WebHome",          Keyboard, KeyCode, WebHome;
    "WebRefresh",       Keyboard, KeyCode, WebRefresh;
    "WebSearch",        Keyboard, KeyCode, WebSearch;
    "WebStop",          Keyboard, KeyCode, WebStop;
    "Yen",              Keyboard, KeyCode, Yen;
    "Copy",             Keyboard, KeyCode, Copy;
    "Paste",            Keyboard, KeyCode, Paste;
    "Cut",              Keyboard, KeyCode, Cut;

    "MouseLeft",   Mouse, MouseButton, Left;
    "MouseRight",  Mouse, MouseButton, Right;
    "MouseMiddle", Mouse, MouseButton, Middle;
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
