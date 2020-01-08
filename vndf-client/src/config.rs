use std::{
    io::{
        self,
        prelude::*,
    },
    fs::File,
    path::Path,
};

use ggez::event::KeyCode;
use serde::{
    Deserialize,
    Serialize,
};


#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub left:   KeyCode,
    pub right:  KeyCode,
    pub thrust: KeyCode,
    pub launch: KeyCode
}

impl Config {
    pub fn load() -> Result<Self, Error> {
        let path = Path::new("config.toml");

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
            left:   KeyCode::Left,
            right:  KeyCode::Right,
            thrust: KeyCode::Up,
            launch: KeyCode::Return,
        }
    }
}


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
