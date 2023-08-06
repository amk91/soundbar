use std::{
    sync::{Arc, Mutex},
};

use crossbeam::channel::{Sender, Receiver};

use super::{
    Soundbites,
    SoundbitesKeyTasks,
    SoundManagerError,
    SoundbiteData,
};

pub enum Message {
    NewSoundbite(SoundbiteData),
}

pub struct SoundState {
    pub soundbites: Arc<Mutex<Soundbites>>,
    pub soundbites_keytasks: Arc<Mutex<SoundbitesKeyTasks>>,

    pub messages: Sender<Message>,
    pub responses: Receiver<Result<String, SoundManagerError>>,
}

impl SoundState {
    pub fn new(
        soundbites: Arc<Mutex<Soundbites>>,
        soundbites_keytasks: Arc<Mutex<SoundbitesKeyTasks>>,
        messages: Sender<Message>,
        responses: Receiver<Result<String, SoundManagerError>>,
    ) -> SoundState {
        SoundState {
            soundbites,
            soundbites_keytasks,
            messages,
            responses,
        }
    }
}
