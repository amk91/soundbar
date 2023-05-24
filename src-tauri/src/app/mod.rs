use std::{
    path::PathBuf,
    collections::HashMap,
};

use tauri::Manager;
use crossbeam::channel::Receiver;
use rodio::{OutputStreamHandle, OutputStream};
use anyhow::{Result, bail};
use log::trace;

#[macro_use]
pub mod commands;
use commands::utils::{Command, CommandPayload, CommandError, CommandResult};

pub mod soundbite;
use soundbite::Soundbite;

pub mod key_task;
use key_task::KeyTask;

mod key_hook;
use key_hook::{KEY_TASK, init_key_hook};
use tauri::AppHandle;

//TODO: error handling
pub struct App {
    root_folder: PathBuf,

    stream_handle: OutputStreamHandle,
    _stream: OutputStream,
    
    soundbites: HashMap<String, Soundbite>,
    soundtasks: HashMap<u16, String>,

    app_handle: AppHandle,
    receiver: Receiver<Command>,
}

impl App {
    pub fn new(
        root_folder: PathBuf,
        receiver: Receiver<Command>,
        app_handle: AppHandle,
    ) -> App {
        init_key_hook();

        //TODO: give option to select a different output device
        let (_stream, stream_handle) = match OutputStream::try_default() {
            Ok(output) => output,
            Err(err) => {
                trace!("ERR: unable to get default output stream, {}", err.to_string());
                panic!("Unable to get default output stream");
            }
        };

        App {
            root_folder,

            stream_handle,
            _stream,
            soundbites: HashMap::new(),
            soundtasks: HashMap::new(),

            app_handle,
            receiver
        }
    }

    pub fn run(&mut self) {
        loop {
            if let Ok(key_task) = KEY_TASK.try_lock().as_deref_mut() {
                if let Some(_) = &key_task.key {
                    if let Some(soundbite_name) = self.soundtasks.get(&key_task.get_code()) {
                        match self.play_soundbite(soundbite_name) {
                            Ok(_) => trace!("Soundbite {} played", soundbite_name),
                            Err(err) => trace!(
                                "ERR: Soundbite couldn't be played [[{}]]",
                                err.to_string()
                            ),
                        }
                    }

                    key_task.key = None;
                }
            }

            if let Ok(command) = self.receiver.try_recv() {
                let _res = match command {
                    Command::Add(name, path) => self.add_soundbite(name, &PathBuf::from(path)),
                    Command::Link(name, key_code) => {
                        match KeyTask::try_from(key_code) {
                            Ok(key_task) => self.link_soundbite_to_keytask(name, key_task),
                            Err(_) => Err(CommandError::KeyCombinationInvalid),
                        }
                    },
                    Command::Volume(name, volume) => self.set_volume(name, volume),
                    Command::Speed(name, speed) => self.set_speed(name, speed),

                    _ => Err(CommandError::UnrecognizedCommand),
                };

                match self.app_handle.emit_all("command_result", CommandPayload::default()) {
                    Ok(_) => trace!("Event emitted"),
                    Err(err) => trace!("ERR: Unable to emit event [[{}]]", err.to_string()),
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }

    fn add_soundbite(
        &mut self,
        name: String,
        path: &PathBuf
    ) -> CommandResult {
        if let Some(_) = self.soundbites.get(&name) {
            trace!("ERR: soundbite name {} [[path: {}]] already in use", name, path.display());
            return Err(CommandError::SoundbiteNameUsed(name));
        }

        if let Ok(soundbite) = Soundbite::new(&self.stream_handle, &path) {
            trace!("Soundbite {} generated, filepath {:?}", name, path);
            self.soundbites.insert(name, soundbite);
            Ok(CommandPayload::default())
        } else {
            trace!("ERR: unable to generate soundbite from path {}", path.display());
            Err(
                CommandError::SoundbiteGenerationFailed(
                    String::from(path.to_str().unwrap_or_default())
                )
            )
        }
    }

    fn link_soundbite_to_keytask(
        &mut self,
        soundbite_name: String,
        key_task: KeyTask
    ) -> CommandResult {
        let soundbite = &self.soundbites.get(&soundbite_name);
        if key_task.get_code() != 0 && soundbite.is_some() {
            if let Some(soundbite_name) = self.soundtasks.get(&key_task.get_code()) {
                trace!("ERR: Key {:?} already assigned to soundbite {}", key_task, soundbite_name);
                return Err(CommandError::KeyAlreadyAssigned(soundbite_name.clone(), "".into()));
            }

            self.soundtasks.insert(key_task.get_code(), soundbite_name);
            return Ok(CommandPayload::default());
        } else if key_task.get_code() == 0 {
            trace!("ERR: Invalid key combination, {} - {:?}", soundbite_name, key_task);
            Err(CommandError::KeyCombinationInvalid)
        } else {
            trace!("Unable to find soundbite {}", soundbite_name);
            Err(CommandError::SoundbiteNotFound(soundbite_name))
        }
    }

    fn set_volume(
        &mut self,
        soundbite_name: String,
        volume: f32
    ) -> CommandResult {
        match self.soundbites.get_mut(&soundbite_name) {
            Some(soundbite) => {
                soundbite.set_volume(volume);
                Ok(CommandPayload::default())
            },
            None => Err(CommandError::SoundbiteNotFound(soundbite_name))
        }
    }

    fn set_speed(
        &mut self,
        soundbite_name: String,
        speed: f32
    ) -> CommandResult {
        match self.soundbites.get_mut(&soundbite_name) {
            Some(soundbite) => {
                soundbite.set_speed(speed);
                Ok(CommandPayload::default())
            },
            None => Err(CommandError::SoundbiteNotFound(soundbite_name))
        }
    }

    fn play_soundbite(&self, soundbite_name: &String) -> Result<()> {
        if let Some(soundbite) = &self.soundbites.get(soundbite_name) {
            soundbite.play();
            Ok(())
        } else {
            trace!("Unable to find soundbite {}", soundbite_name);
            bail!("Unable to find soundbite {}", soundbite_name);
        }
    }
}
