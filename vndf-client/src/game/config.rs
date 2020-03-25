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
    ($($s:tt => $k:tt,)*) => {
        impl Serialize for Key {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: Serializer
            {
                #[allow(unused_parens)]
                let expr = match self {
                    $($k => $s,)*

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
                    $($s => Ok($k),)*
                    
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
                    $($k => write!(f, "{}", $s),)*

                    _ => write!(f, "Unknown key"),
                }
            }
        }
    };
}

keys!(
    "Key1"             => (Key::Keyboard(KeyCode::Key1)),
    "Key2"             => (Key::Keyboard(KeyCode::Key2)),
    "Key3"             => (Key::Keyboard(KeyCode::Key3)),
    "Key4"             => (Key::Keyboard(KeyCode::Key4)),
    "Key5"             => (Key::Keyboard(KeyCode::Key5)),
    "Key6"             => (Key::Keyboard(KeyCode::Key6)),
    "Key7"             => (Key::Keyboard(KeyCode::Key7)),
    "Key8"             => (Key::Keyboard(KeyCode::Key8)),
    "Key9"             => (Key::Keyboard(KeyCode::Key9)),
    "Key0"             => (Key::Keyboard(KeyCode::Key0)),
    "A"                => (Key::Keyboard(KeyCode::A)),
    "B"                => (Key::Keyboard(KeyCode::B)),
    "C"                => (Key::Keyboard(KeyCode::C)),
    "D"                => (Key::Keyboard(KeyCode::D)),
    "E"                => (Key::Keyboard(KeyCode::E)),
    "F"                => (Key::Keyboard(KeyCode::F)),
    "G"                => (Key::Keyboard(KeyCode::G)),
    "H"                => (Key::Keyboard(KeyCode::H)),
    "I"                => (Key::Keyboard(KeyCode::I)),
    "J"                => (Key::Keyboard(KeyCode::J)),
    "K"                => (Key::Keyboard(KeyCode::K)),
    "L"                => (Key::Keyboard(KeyCode::L)),
    "M"                => (Key::Keyboard(KeyCode::M)),
    "N"                => (Key::Keyboard(KeyCode::N)),
    "O"                => (Key::Keyboard(KeyCode::O)),
    "P"                => (Key::Keyboard(KeyCode::P)),
    "Q"                => (Key::Keyboard(KeyCode::Q)),
    "R"                => (Key::Keyboard(KeyCode::R)),
    "S"                => (Key::Keyboard(KeyCode::S)),
    "T"                => (Key::Keyboard(KeyCode::T)),
    "U"                => (Key::Keyboard(KeyCode::U)),
    "V"                => (Key::Keyboard(KeyCode::V)),
    "W"                => (Key::Keyboard(KeyCode::W)),
    "X"                => (Key::Keyboard(KeyCode::X)),
    "Y"                => (Key::Keyboard(KeyCode::Y)),
    "Z"                => (Key::Keyboard(KeyCode::Z)),
    "Esc"              => (Key::Keyboard(KeyCode::Escape)),
    "F1"               => (Key::Keyboard(KeyCode::F1)),
    "F2"               => (Key::Keyboard(KeyCode::F2)),
    "F3"               => (Key::Keyboard(KeyCode::F3)),
    "F4"               => (Key::Keyboard(KeyCode::F4)),
    "F5"               => (Key::Keyboard(KeyCode::F5)),
    "F6"               => (Key::Keyboard(KeyCode::F6)),
    "F7"               => (Key::Keyboard(KeyCode::F7)),
    "F8"               => (Key::Keyboard(KeyCode::F8)),
    "F9"               => (Key::Keyboard(KeyCode::F9)),
    "F10"              => (Key::Keyboard(KeyCode::F10)),
    "F11"              => (Key::Keyboard(KeyCode::F11)),
    "F12"              => (Key::Keyboard(KeyCode::F12)),
    "F13"              => (Key::Keyboard(KeyCode::F13)),
    "F14"              => (Key::Keyboard(KeyCode::F14)),
    "F15"              => (Key::Keyboard(KeyCode::F15)),
    "F16"              => (Key::Keyboard(KeyCode::F16)),
    "F17"              => (Key::Keyboard(KeyCode::F17)),
    "F18"              => (Key::Keyboard(KeyCode::F18)),
    "F19"              => (Key::Keyboard(KeyCode::F19)),
    "F20"              => (Key::Keyboard(KeyCode::F20)),
    "F21"              => (Key::Keyboard(KeyCode::F21)),
    "F22"              => (Key::Keyboard(KeyCode::F22)),
    "F23"              => (Key::Keyboard(KeyCode::F23)),
    "F24"              => (Key::Keyboard(KeyCode::F24)),
    "Snapshot"         => (Key::Keyboard(KeyCode::Snapshot)),
    "Scroll"           => (Key::Keyboard(KeyCode::Scroll)),
    "Pause"            => (Key::Keyboard(KeyCode::Pause)),
    "Insert"           => (Key::Keyboard(KeyCode::Insert)),
    "Home"             => (Key::Keyboard(KeyCode::Home)),
    "Delete"           => (Key::Keyboard(KeyCode::Delete)),
    "End"              => (Key::Keyboard(KeyCode::End)),
    "PageDown"         => (Key::Keyboard(KeyCode::PageDown)),
    "PageUp"           => (Key::Keyboard(KeyCode::PageUp)),
    "Left"             => (Key::Keyboard(KeyCode::Left)),
    "Up"               => (Key::Keyboard(KeyCode::Up)),
    "Right"            => (Key::Keyboard(KeyCode::Right)),
    "Down"             => (Key::Keyboard(KeyCode::Down)),
    "Back"             => (Key::Keyboard(KeyCode::Back)),
    "Return"           => (Key::Keyboard(KeyCode::Return)),
    "Space"            => (Key::Keyboard(KeyCode::Space)),
    "Compose"          => (Key::Keyboard(KeyCode::Compose)),
    "Caret"            => (Key::Keyboard(KeyCode::Caret)),
    "Numlock"          => (Key::Keyboard(KeyCode::Numlock)),
    "Numpad0"          => (Key::Keyboard(KeyCode::Numpad0)),
    "Numpad1"          => (Key::Keyboard(KeyCode::Numpad1)),
    "Numpad2"          => (Key::Keyboard(KeyCode::Numpad2)),
    "Numpad3"          => (Key::Keyboard(KeyCode::Numpad3)),
    "Numpad4"          => (Key::Keyboard(KeyCode::Numpad4)),
    "Numpad5"          => (Key::Keyboard(KeyCode::Numpad5)),
    "Numpad6"          => (Key::Keyboard(KeyCode::Numpad6)),
    "Numpad7"          => (Key::Keyboard(KeyCode::Numpad7)),
    "Numpad8"          => (Key::Keyboard(KeyCode::Numpad8)),
    "Numpad9"          => (Key::Keyboard(KeyCode::Numpad9)),
    "AbntC1"           => (Key::Keyboard(KeyCode::AbntC1)),
    "AbntC2"           => (Key::Keyboard(KeyCode::AbntC2)),
    "Add"              => (Key::Keyboard(KeyCode::Add)),
    "Apostrophe"       => (Key::Keyboard(KeyCode::Apostrophe)),
    "Apps"             => (Key::Keyboard(KeyCode::Apps)),
    "At"               => (Key::Keyboard(KeyCode::At)),
    "Ax"               => (Key::Keyboard(KeyCode::Ax)),
    "Backslash"        => (Key::Keyboard(KeyCode::Backslash)),
    "Calculator"       => (Key::Keyboard(KeyCode::Calculator)),
    "Capital"          => (Key::Keyboard(KeyCode::Capital)),
    "Colon"            => (Key::Keyboard(KeyCode::Colon)),
    "Comma"            => (Key::Keyboard(KeyCode::Comma)),
    "Convert"          => (Key::Keyboard(KeyCode::Convert)),
    "Decimal"          => (Key::Keyboard(KeyCode::Decimal)),
    "Divide"           => (Key::Keyboard(KeyCode::Divide)),
    "Equals"           => (Key::Keyboard(KeyCode::Equals)),
    "Grave"            => (Key::Keyboard(KeyCode::Grave)),
    "Kana"             => (Key::Keyboard(KeyCode::Kana)),
    "Kanji"            => (Key::Keyboard(KeyCode::Kanji)),
    "LAlt"             => (Key::Keyboard(KeyCode::LAlt)),
    "LBracket"         => (Key::Keyboard(KeyCode::LBracket)),
    "LControl"         => (Key::Keyboard(KeyCode::LControl)),
    "LShift"           => (Key::Keyboard(KeyCode::LShift)),
    "LWin"             => (Key::Keyboard(KeyCode::LWin)),
    "Mail"             => (Key::Keyboard(KeyCode::Mail)),
    "MediaSelect"      => (Key::Keyboard(KeyCode::MediaSelect)),
    "MediaStop"        => (Key::Keyboard(KeyCode::MediaStop)),
    "Minus"            => (Key::Keyboard(KeyCode::Minus)),
    "Multiply"         => (Key::Keyboard(KeyCode::Multiply)),
    "Mute"             => (Key::Keyboard(KeyCode::Mute)),
    "MyComputer"       => (Key::Keyboard(KeyCode::MyComputer)),
    "NavigateForward"  => (Key::Keyboard(KeyCode::NavigateForward)),
    "NavigateBackward" => (Key::Keyboard(KeyCode::NavigateBackward)),
    "NextTrack"        => (Key::Keyboard(KeyCode::NextTrack)),
    "NoConvert"        => (Key::Keyboard(KeyCode::NoConvert)),
    "NumpadComma"      => (Key::Keyboard(KeyCode::NumpadComma)),
    "NumpadEnter"      => (Key::Keyboard(KeyCode::NumpadEnter)),
    "NumpadEquals"     => (Key::Keyboard(KeyCode::NumpadEquals)),
    "OEM102"           => (Key::Keyboard(KeyCode::OEM102)),
    "Period"           => (Key::Keyboard(KeyCode::Period)),
    "PlayPause"        => (Key::Keyboard(KeyCode::PlayPause)),
    "Power"            => (Key::Keyboard(KeyCode::Power)),
    "PrevTrack"        => (Key::Keyboard(KeyCode::PrevTrack)),
    "RAlt"             => (Key::Keyboard(KeyCode::RAlt)),
    "RBracket"         => (Key::Keyboard(KeyCode::RBracket)),
    "RControl"         => (Key::Keyboard(KeyCode::RControl)),
    "RShift"           => (Key::Keyboard(KeyCode::RShift)),
    "RWin"             => (Key::Keyboard(KeyCode::RWin)),
    "Semicolon"        => (Key::Keyboard(KeyCode::Semicolon)),
    "Slash"            => (Key::Keyboard(KeyCode::Slash)),
    "Sleep"            => (Key::Keyboard(KeyCode::Sleep)),
    "Stop"             => (Key::Keyboard(KeyCode::Stop)),
    "Subtract"         => (Key::Keyboard(KeyCode::Subtract)),
    "Sysrq"            => (Key::Keyboard(KeyCode::Sysrq)),
    "Tab"              => (Key::Keyboard(KeyCode::Tab)),
    "Underline"        => (Key::Keyboard(KeyCode::Underline)),
    "Unlabeled"        => (Key::Keyboard(KeyCode::Unlabeled)),
    "VolumeDown"       => (Key::Keyboard(KeyCode::VolumeDown)),
    "VolumeUp"         => (Key::Keyboard(KeyCode::VolumeUp)),
    "Wake"             => (Key::Keyboard(KeyCode::Wake)),
    "WebBack"          => (Key::Keyboard(KeyCode::WebBack)),
    "WebFavorites"     => (Key::Keyboard(KeyCode::WebFavorites)),
    "WebForward"       => (Key::Keyboard(KeyCode::WebForward)),
    "WebHome"          => (Key::Keyboard(KeyCode::WebHome)),
    "WebRefresh"       => (Key::Keyboard(KeyCode::WebRefresh)),
    "WebSearch"        => (Key::Keyboard(KeyCode::WebSearch)),
    "WebStop"          => (Key::Keyboard(KeyCode::WebStop)),
    "Yen"              => (Key::Keyboard(KeyCode::Yen)),
    "Copy"             => (Key::Keyboard(KeyCode::Copy)),
    "Paste"            => (Key::Keyboard(KeyCode::Paste)),
    "Cut"              => (Key::Keyboard(KeyCode::Cut)),
    "MouseLeft"        => (Key::Mouse(MouseButton::Left)),
    "MouseRight"       => (Key::Mouse(MouseButton::Right)),
    "MouseMiddle"      => (Key::Mouse(MouseButton::Middle)),
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
