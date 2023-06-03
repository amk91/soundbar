use std::{
    sync::{Arc, Mutex},
};

use crossbeam::channel::{Sender, Receiver};

use super::{
    Soundbites,
    SoundbitesKeyTasks,
    NewSoundbiteMessage,
    SoundManagerError,
};

pub struct SoundState {
    pub soundbites: Arc<Mutex<Soundbites>>,
    pub soundbites_keytasks: Arc<Mutex<SoundbitesKeyTasks>>,

    pub new_soundbite: Sender<NewSoundbiteMessage>,
    pub new_soundbite_ack: Receiver<Result<String, SoundManagerError>>,
}

impl SoundState {
    pub fn new(
        soundbites: Arc<Mutex<Soundbites>>,
        soundbites_keytasks: Arc<Mutex<SoundbitesKeyTasks>>,    
        new_soundbite: Sender<NewSoundbiteMessage>,
        new_soundbite_ack: Receiver<Result<String, SoundManagerError>>,
    ) -> SoundState {
        SoundState {
            soundbites,
            soundbites_keytasks,
            new_soundbite,
            new_soundbite_ack
        }
    }
}
