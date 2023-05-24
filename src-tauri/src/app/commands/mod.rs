use tauri::State;
use anyhow::Result;
use log::trace;

pub mod utils;
use utils::*;

//TODO: make tauri::commands return results

fn send_command(
    state: &State<'_, CommandState>,
    command: Command
) -> Result<(), CommandError> {
    trace!("Command fired {:?}", command);
    match state.sender.lock() {
        Ok(sender) => {
            match sender.send(command.clone()) {
                Ok(_) => Ok(()),
                Err(err) => Err(CommandError::Send(
                    command,
                    err.to_string())
                ),
            }
        },
        Err(err) => Err(CommandError::SenderMutex(
            command,
            err.to_string())
        )
    }
}

#[tauri::command]
pub fn get_key(key: u32, sys_key: u32) {
    println!("KEY RECEIVED: {}-{}", key, sys_key);
}

#[tauri::command]
pub fn add_soundbite(
    soundbite_name: String,
    path: String,
    state: State<'_, CommandState>
) -> Result<(), CommandError> {
    let command = Command::Add(soundbite_name, path);
    send_command(&state, command)
}

#[tauri::command]
pub fn set_soundbite_volume(
    soundbite_name: String,
    volume: f32,
    state: State<'_, CommandState>
) -> Result<(), CommandError> {
    let command = Command::Volume(soundbite_name, volume);
    send_command(&state, command)
}

#[tauri::command]
pub fn set_soundbite_speed(
    soundbite_name: String,
    speed: f32,
    state: State<'_, CommandState>
) -> Result<(), CommandError> {
    let command = Command::Speed(soundbite_name, speed);
    send_command(&state, command)
}

#[tauri::command]
pub fn link_soundbite_to_keytask(
    soundbite_name: String,
    key_code: u32,
    state: State<'_, CommandState>
) -> Result<(), CommandError> {
    let command = Command::Link(soundbite_name, key_code);
    send_command(&state, command)
}

#[tauri::command]
pub fn get_soundbites_list() -> Vec<String> {
    panic!("get_soundbites_list not implemented");
}

#[tauri::command]
pub fn get_linked_soundbites_list() -> Vec<(u32, String)> {
    panic!("get_linked_soundbites_list not implemented");
}
