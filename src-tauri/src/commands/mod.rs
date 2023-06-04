use tauri::State;
use anyhow::Result;
use log::error;

use super::soundmanager::{
    soundstate::SoundState,
    soundbite::SoundbiteData,
    key_task::KeyTaskCode,
    utils::{
        SoundManagerError,
        NewSoundbiteError,
        NewSoundbiteMessage,
    }
};

#[tauri::command]
pub fn add_soundbite(
    buffer: Vec<u8>,
    name: String,
    volume: f32,
    speed: f32,
    keycode: KeyTaskCode,
    state: State<'_, SoundState>
) -> Result<String, SoundManagerError> {
    let data = SoundbiteData::new(buffer, volume, speed).unwrap();
    if let Err(err) = state.new_soundbite.send(NewSoundbiteMessage {
        data,
        name: name.clone(),
        keycode: if keycode == 0 { None } else { Some(keycode) }
    }) {
        error!(
            "Unable to send command to add soundbite named {} [[{:?}]]",
            name.clone(),
            err
        );
        return Err(SoundManagerError::NewSoundbiteError(
            NewSoundbiteError::UnableToSendSoundbite(name.clone())
        ));
    }

    match state.new_soundbite_ack.recv() {
        Ok(message) => return message,
        Err(err) => {
            error!(
                "Unable to receive command to add soundbite named {} [[{:?}]]",
                name.clone(),
                err
            );
            return Err(SoundManagerError::NewSoundbiteError(
                NewSoundbiteError::UnableToSendSoundbite(name.clone())
            ));
        }
    }
}

#[tauri::command]
pub fn remove_soundbite(
    name: String,
    state: State<'_, SoundState>,
) -> Result<(), SoundManagerError> {
    let mut soundbites = state.soundbites.lock().unwrap();
    match soundbites.iter().position(|s| s.name == name) {
        Some(index) => {
            let mut soundbites_keytasks = state.soundbites_keytasks.lock().unwrap();
            let keys: Vec<KeyTaskCode> = soundbites_keytasks.iter().filter_map(
                |(&key, &val)| if val == index { Some(key) } else { None }
            ).collect();
            keys.iter().for_each(
                |keycode| {
                    soundbites_keytasks.remove(keycode);
                }
            );

            soundbites.remove(index);

            Ok(())
        }
        None => Err(SoundManagerError::SoundbiteNotFound(name)),
    }
}

#[tauri::command]
pub fn set_volume(
    name: String,
    volume: f32,
    state: State<'_, SoundState>,
) -> Result<(), SoundManagerError> {
    if volume <= 0f32 || volume > 200f32 {
        return Err(SoundManagerError::InvalidVolumeValue);
    }

    let mut soundbites = state.soundbites.lock().unwrap();
    match soundbites.iter().position(|s| s.name == name) {
        Some(index) => {
            soundbites[index].set_volume(volume);
            Ok(())
        },
        None => Err(SoundManagerError::SoundbiteNotFound(name)),
    }
}

#[tauri::command]
pub fn set_speed(
    name: String,
    speed: f32,
    state: State<'_, SoundState>,
) -> Result<(), SoundManagerError> {
    if speed <= 0f32 || speed > 200f32 {
        return Err(SoundManagerError::InvalidSpeedValue);
    }

    let mut soundbites = state.soundbites.lock().unwrap();
    match soundbites.iter().position(|s| s.name == name) {
        Some(index) => {
            soundbites[index].set_speed(speed);
            Ok(())
        },
        None => Err(SoundManagerError::SoundbiteNotFound(name)),
    }
}
