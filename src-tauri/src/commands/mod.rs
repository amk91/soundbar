use tauri::State;
use anyhow::Result;
use log::error;

use crate::soundmanager::{SoundManager, key_task};

use super::soundmanager::{
    soundstate::SoundState,
    soundbite::SoundbiteData,
    key_task::KeyTaskCode,
    utils::{
        SoundManagerError,
        NewSoundbiteError,
        NewSoundbiteMessage,
        SoundbiteInfo,
    }
};

#[tauri::command]
pub fn add_soundbite(
    buffer: Vec<u8>,
    name: String,
    state: State<'_, SoundState>
) -> Result<String, SoundManagerError> {
    let data = SoundbiteData::new(buffer, 100f32, 100f32).unwrap();
    if let Err(err) = state.new_soundbite.send(NewSoundbiteMessage {
        data,
        name: name.clone(),
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
pub fn set_name(
    name: String,
    new_name: String,
    state: State<'_, SoundState>,
) -> Result<(), SoundManagerError> {
    let mut soundbites = state.soundbites.lock().unwrap();
    match soundbites.iter().find(|soundbite| soundbite.name == new_name) {
        Some(_) => Err(SoundManagerError::SoundbiteAlreadyExists(new_name)),
        None => {
            match soundbites.iter().position(|soundbite| soundbite.name == name) {
                Some(index) => {
                    soundbites[index].name = new_name;
                    Ok(())
                },
                None => Err(SoundManagerError::SoundbiteNotFound(name)),
            }
        }
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

#[tauri::command]
pub fn set_keytask_code(
    name: String,
    keytask_code: KeyTaskCode,
    state: State<'_, SoundState>
) -> Result<(), SoundManagerError> {
    let soundbites = state.soundbites.lock().unwrap();
    if let Some(index) = soundbites.iter().position(|soundbite| soundbite.name == name) {
        let mut soundbites_keytasks = state.soundbites_keytasks.lock().unwrap();
        if let Some(_) = soundbites_keytasks.iter().find(|keytask| *keytask.1 == index) {
            return Err(SoundManagerError::KeyTaskAlreadyAssignedToSoundbite(keytask_code, name));
        }

        if let Some(_) = soundbites_keytasks.get(&keytask_code) {
            return Err(SoundManagerError::KeyTaskUsed(keytask_code));
        }

        soundbites_keytasks.insert(keytask_code, index);
        Ok(())
    } else {
        Err(SoundManagerError::SoundbiteNotFound(name))
    }
}

#[tauri::command]
pub fn remove_keytask_code(
    name: String,
    state: State<'_, SoundState>
) -> Result<(), SoundManagerError> {
    let soundbites = state.soundbites.lock().unwrap();
    if let Some(index) = soundbites.iter().position(|soundbite| soundbite.name == name) {
        let mut soundbites_keytasks = state.soundbites_keytasks.lock().unwrap();
        let keys: Vec<KeyTaskCode> = soundbites_keytasks.iter().filter_map(
            |(&key, &val)| if val == index { Some(key) } else { None }
        ).collect();
        keys.iter().for_each(
            |keycode| {
                soundbites_keytasks.remove(keycode);
            }
        );
    }

    Ok(())
}

#[tauri::command]
pub fn get_soundbite(
    name: String,
    state: State<'_, SoundState>
) -> Result<SoundbiteInfo, SoundManagerError> {
    let soundbites = state.soundbites.lock().unwrap();
    match soundbites.iter().position(|soundbite| *soundbite.name == name) {
        Some(index) => {
            let soundbites_keytasks = state.soundbites_keytasks.lock().unwrap();
            let keycode = match soundbites_keytasks.iter().find(|(_, &value)| {
                value == index
            }) {
                Some(item) => item.0.clone(),
                None => 0
            };

            let soundbite = &soundbites[index];
            Ok(SoundbiteInfo {
                name: soundbite.name.clone(),
                volume: soundbite.data.volume,
                speed: soundbite.data.speed,
                keycode
            })
        },
        None => {
            Err(SoundManagerError::SoundbiteNotFound(name.clone()))
        }
    }
}

#[tauri::command]
pub fn get_soundbites(
    state: State<'_, SoundState>
) -> Vec<String> {
    let soundbites = state.soundbites.lock().unwrap();
    soundbites.iter().map(|soundbite| soundbite.name.clone()).collect()
}
