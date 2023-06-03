use super::{KeyTaskCode, SoundbiteData};

use thiserror::Error;
use serde::Serialize;

pub struct NewSoundbiteMessage {
    pub data: SoundbiteData,
    pub name: String,
    pub keycode: Option<KeyTaskCode>,
}

// pub struct SoundbitesListItem {
//     pub name: String,
//     pub volume: f32,
//     pub speed: f32,
//     pub keycode: KeyTaskCode,
// }

#[derive(Error, Debug, Clone, Serialize)]
pub enum NewSoundbiteError {
    #[error("Unable to create soundbite")]
    FailOnCreate,
    #[error("Name {0} already used")]
    NameUsed(String),
    #[error("Key code {0} already used")]
    KeyTaskUsed(KeyTaskCode),
    #[error("Unable to send soundbite {0} to backend")]
    UnableToSendSoundbite(String),
}

#[derive(Error, Debug, Clone, Serialize)]
pub enum SoundManagerError {
    #[error(transparent)]
    NewSoundbiteError(NewSoundbiteError),
    #[error("Soundbite named {0} not found")]
    SoundbiteNotFound(String),
    #[error("Invalid volume value")]
    InvalidVolumeValue,
    #[error("Invalid speed value")]
    InvalidSpeedValue,
}
