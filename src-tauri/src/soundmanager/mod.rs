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
use once_cell::sync::Lazy;


pub mod key_hook;
pub mod key_task;
pub mod soundbite;
pub mod soundstate;
pub mod utils;

use soundbite::{Soundbite, SoundbiteData};
use utils::{NewSoundbiteError, SoundManagerError};

use key_hook::{KEY_TASK, init_key_hook};
use key_task::KeyTaskCode;

use self::soundstate::Message;

pub type Soundbites = Vec<Soundbite>;
pub type SoundbitesKeyTasks = HashMap<KeyTaskCode, usize>;

pub const SOUNDBITES_FILE: &str = "sdata.dat";
pub const KEYTASKS_FILE: &str = "kdata.dat";
pub static ROOT_FOLDER: Lazy<Mutex<PathBuf>> = Lazy::new(|| Mutex::new(PathBuf::from("")));

pub struct SoundManager {
    messages: Receiver<Message>,
    responses: Sender<Result<String, SoundManagerError>>,

    stream_handle: OutputStreamHandle,
    //OutputStream needs to be kept alive in order for the sound to be played
    _stream: OutputStream,

    soundbites: Arc<Mutex<Soundbites>>,
    soundbites_keytasks: Arc<Mutex<SoundbitesKeyTasks>>,
}

impl SoundManager {
    pub fn new(
        messages: Receiver<Message>,
        responses: Sender<Result<String, SoundManagerError>>,

        soundbites: Arc<Mutex<Soundbites>>,
        soundbites_keytasks: Arc<Mutex<SoundbitesKeyTasks>>,
    ) -> SoundManager {
        init_key_hook();

        let (_stream, stream_handle) = match OutputStream::try_default() {
            Ok(output) => output,
            Err(err) => panic!("Unable to get default output stream [[{:?}]]", err),
        };

        for soundbite in soundbites.lock().unwrap().iter_mut() {
            soundbite.init_sink(&stream_handle).map_err(|err| error!(
                "Unable to init sink for soundbite named {} [[{:?}]]",
                soundbite.data.name.clone(),
                err,
            )).unwrap();
        }

        SoundManager {
            messages,
            responses,

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

            if let Ok(message) = self.messages.try_recv() {
                match message {
                    Message::NewSoundbite(data) => {
                        self.responses.send(
                            self.add_soundbite(data)
                        ).map_err(
                            |err| error!("Unable to send ack for new soundbite [[{:?}]]", err)
                        ).unwrap();
                    },
                }
            }

            thread::sleep(Duration::from_millis(100));
        }
    }

    fn play_soundbite(&self, key_task_code: KeyTaskCode) -> Result<()> {
        if let (Ok(soundbites), Ok(soundbites_keytasks)) = (
            self.soundbites.try_lock(),
            self.soundbites_keytasks.try_lock()
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
        soundbite_data: SoundbiteData
    ) -> Result<String, SoundManagerError> {
        let mut soundbites = self.soundbites.lock().unwrap();
        if let Some(_) = soundbites.iter().position(|s| s.data.name == soundbite_data.name) {
            error!("Soundbite named {} already exists", soundbite_data.name);
            return Err(SoundManagerError::NewSoundbiteError(
                NewSoundbiteError::NameUsed(soundbite_data.name)
            ));
        }

        let soundbite_name = soundbite_data.name.clone();
        let soundbite = match Soundbite::new(
            &self.stream_handle,
            soundbite_data
        ) {
            Ok(soundbite) => soundbite,
            Err(err) => {
                error!(
                    "Unable to generate soundbite named {soundbite_name} [[{:?}]]",
                    err
                );
                return Err(SoundManagerError::NewSoundbiteError(
                    NewSoundbiteError::FailOnCreate
                ));
            }
        };

        soundbites.push(soundbite);
        Ok(soundbite_name)
    }
}
