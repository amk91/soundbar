use std::{
    sync::{Arc, Mutex},
    collections::HashMap,
    thread,
    time::Duration,
    path::PathBuf,
};

use anyhow::{Result, bail};
use crossbeam::channel::{Receiver, Sender};
use rodio::{OutputStreamHandle, OutputStream};
use log::{trace, error};

pub mod key_hook;
pub mod key_task;
pub mod soundbite;
pub mod soundstate;
pub mod utils;

use soundbite::{Soundbite, SoundbiteData};
use utils::{NewSoundbiteError, NewSoundbiteMessage, SoundManagerError};

use key_hook::{KEY_TASK, init_key_hook};
use key_task::KeyTaskCode;

pub type Soundbites = Vec<Soundbite>;
pub type SoundbitesKeyTasks = HashMap<KeyTaskCode, usize>;

pub struct SoundManager {
    root_folder: PathBuf,

    new_soundbite: Receiver<NewSoundbiteMessage>,
    new_soundbite_ack: Sender<Result<String, SoundManagerError>>,

    stream_handle: OutputStreamHandle,
    _stream: OutputStream,

    soundbites: Arc<Mutex<Soundbites>>,
    soundbites_keytasks: Arc<Mutex<SoundbitesKeyTasks>>,
}

impl SoundManager {
    pub fn new(
        root_folder: PathBuf,

        new_soundbite: Receiver<NewSoundbiteMessage>,
        new_soundbite_ack: Sender<Result<String, SoundManagerError>>,

        soundbites: Arc<Mutex<Soundbites>>,
        soundbites_keytasks: Arc<Mutex<SoundbitesKeyTasks>>,
    ) -> SoundManager {
        init_key_hook();

        let (_stream, stream_handle) = match OutputStream::try_default() {
            Ok(output) => output,
            Err(err) => panic!("Unable to get default output stream [[{:?}]]", err),
        };

        SoundManager {
            root_folder,

            new_soundbite,
            new_soundbite_ack,

            stream_handle,
            _stream,

            soundbites,
            soundbites_keytasks,
        }
    }

    pub fn run(&self) {
        loop {
            if let Ok(key_task) = KEY_TASK.try_lock().as_deref_mut() {
                if let Some(_) = &key_task.key {
                    if let Ok(_) = self.play_soundbite(key_task.get_code()) {
                        key_task.key = None;
                    }
                }
            }

            // If a request to add a new soundbite is sent,
            // the backend sends back the result of the call add_soundbite
            if let Ok(soundbite_data) = self.new_soundbite.try_recv() {
                self.new_soundbite_ack.send(
                    self.add_soundbite(soundbite_data)
                ).map_err(
                    |err| error!("Unable to send ack for new soundbite [[{:?}]]", err)
                ).unwrap();
            }

            thread::sleep(Duration::from_millis(100));
        }
    }

    fn play_soundbite(&self, key_task_code: KeyTaskCode) -> Result<()> {
        if let (Ok(soundbites), Ok(soundbites_keytasks)) = (
            self.soundbites.clone().try_lock(),
            self.soundbites_keytasks.clone().try_lock()
        ) {
            match soundbites_keytasks.get(&key_task_code) {
                Some(index) => {
                    if let Some(soundbite) = soundbites.get(*index) {
                        soundbite.play();
                        trace!("Soundbite linked to key code {} played", key_task_code);
                        return Ok(());
                    } else {
                        bail!("Soundbites index out of bound")
                    }
                },
                None => {
                    error!("No soundbite linked to key code {}", key_task_code);
                    bail!("No soundbite linked to key code {}", key_task_code)
                },
            }
        } else {
            bail!("Locked")
        }
    }

    fn add_soundbite(
        &self,
        soundbite_data: NewSoundbiteMessage
    ) -> Result<String, SoundManagerError> {
        let mut soundbites = self.soundbites.lock().unwrap();
        if let Some(_) = soundbites.iter().position(|s| s.name == soundbite_data.name) {
            error!("Soundbite named {} already exists", soundbite_data.name);
            return Err(SoundManagerError::NewSoundbiteError(
                NewSoundbiteError::NameUsed(soundbite_data.name)
            ));
        }

        let mut soundbites_keytasks = self.soundbites_keytasks.lock().unwrap();
        if let Some(keycode) = soundbite_data.keycode {
            if let Some(_) = soundbites_keytasks.get(&keycode) {
                error!("Keycode {} already used", keycode);
                return Err(SoundManagerError::NewSoundbiteError(
                    NewSoundbiteError::KeyTaskUsed(keycode)
                ));
            }
        }

        let soundbite = match Soundbite::new(
            &self.stream_handle,
            soundbite_data.name.clone(),
            soundbite_data.data
        ) {
            Ok(soundbite) => soundbite,
            Err(err) => {
                error!(
                    "Unable to generate soundbite named {} [[{:?}]]",
                    soundbite_data.name,
                    err
                );
                return Err(SoundManagerError::NewSoundbiteError(
                    NewSoundbiteError::FailOnCreate
                ));
            }
        };

        soundbites.push(soundbite);
        if let Some(keycode) = soundbite_data.keycode {
            soundbites_keytasks.insert(keycode, soundbites.len() - 1);
        }

        Ok(soundbite_data.name)
    }
}
