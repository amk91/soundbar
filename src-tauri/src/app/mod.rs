use std::{
    path::PathBuf,
    collections::HashMap,
};

use crossbeam::channel::Receiver;
use rodio::{OutputStreamHandle, OutputStream};
use anyhow::{Result, bail, anyhow};
use log::trace;

#[macro_use]
pub mod commands;
use commands::utils::Command;

pub mod soundbite;
use soundbite::Soundbite;

pub mod key_task;
use key_task::KeyTask;

mod key_hook;
use key_hook::{KEY_TASK, init_key_hook};

//TODO: error handling
pub struct App {
    root_folder: PathBuf,

    stream_handle: OutputStreamHandle,
    _stream: OutputStream,
    
    soundbites: HashMap<String, Soundbite>,
    soundtasks: HashMap<u16, String>,

    receiver: Receiver<Command>,
}

impl App {
    pub fn new(root_folder: PathBuf, receiver: Receiver<Command>) -> App {
        init_key_hook();

        //TODO: give option to select a different output device
        let (_stream, stream_handle) = if let Ok(output) = OutputStream::try_default() {
            output
        } else {
            panic!("Unable to get default output stream");
        };

        App {
            root_folder,

            stream_handle,
            _stream,
            soundbites: HashMap::new(),
            soundtasks: HashMap::new(),

            receiver
        }
    }

    pub fn run(&mut self) {
        loop {
            if let Ok(v) = KEY_TASK.try_lock().as_deref_mut() {
                if let Some(_) = &v.key {
                    if let Some(soundbite_name) = self.soundtasks.get(&v.get_code()) {
                        self.play_soundbite(soundbite_name).unwrap_or(());
                    }

                    v.key = None;
                }
            }

            if let Ok(command) = self.receiver.try_recv() {
                let res = match command {
                    Command::Add(name, path) => self.add_soundbite(name, &PathBuf::from(path)),
                    Command::Link(name, key_code) => {
                        match KeyTask::try_from(key_code) {
                            Ok(key_task) => self.link_soundbite_to_keytask(name, key_task),
                            Err(err) => Err(err),
                        }
                    },
                    Command::Volume(name, volume) => self.set_volume(name, volume),
                    Command::Speed(name, speed) => self.set_speed(name, speed),
                    _ => Err(anyhow!("Unrecognized command, {:?}", command)),
                };

                //TODO: display error on UI
                res.unwrap_or(());
            }

            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }

    pub fn add_soundbite(
        &mut self,
        name: String,
        path: &PathBuf
    ) -> Result<()> {
        if let Ok(soundbite) = Soundbite::new(&self.stream_handle, &path) {
            trace!("Soundbite {} generated, filepath {:?}", name, path);
            self.soundbites.insert(name, soundbite);
            return Ok(());
        } else {
            trace!("{} - {:?}", name, path);
            bail!(
                "Unable to generate soundbite from {}",
                path.to_str().unwrap_or("")
            );
        }
    }

    pub fn link_soundbite_to_keytask(
        &mut self,
        soundbite_name: String,
        key_task: KeyTask
    ) -> Result<()> {
        let soundbite = &self.soundbites.get(&soundbite_name);
        if key_task.get_code() != 0 && soundbite.is_some() {
            self.soundtasks.insert(key_task.get_code(), soundbite_name);
            return Ok(());
        } else if key_task.get_code() == 0 {
            trace!("Invalid key combination, {} - {:?}", soundbite_name, key_task);
            bail!("Key combination is invalid or empty");
        } else {
            trace!("Unable to find soundbite {}", soundbite_name);
            bail!("Unable to find soundbite {}", soundbite_name);
        }
    }

    pub fn set_volume(
        &mut self,
        soundbite_name: String,
        volume: f32
    ) -> Result<()> {
        match self.soundbites.get_mut(&soundbite_name) {
            Some(soundbite) => {
                soundbite.set_volume(volume);
                Ok(())
            },
            None => bail!("Unable to find soundbite {}", soundbite_name)
        }
    }

    pub fn set_speed(
        &mut self,
        soundbite_name: String,
        speed: f32
    ) -> Result<()> {
        match self.soundbites.get_mut(&soundbite_name) {
            Some(soundbite) => {
                soundbite.set_speed(speed);
                Ok(())
            },
            None => bail!("Unable to find soundbite {}", soundbite_name)
        }
    }

    pub fn play_soundbite(&self, soundbite_name: &String) -> Result<()> {
        if let Some(soundbite) = &self.soundbites.get(soundbite_name) {
            soundbite.play();
            Ok(())
        } else {
            trace!("Unable to find soundbite {}", soundbite_name);
            bail!("Unable to find soundbite {}", soundbite_name);
        }
    }
}
