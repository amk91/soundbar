use std::sync::Mutex;

use crossbeam::channel::Sender;
use thiserror::Error;

#[derive(Clone, Debug, serde::Serialize)]
pub enum Command {
    Add(String, String),
    Volume(String, f32),
    Speed(String, f32),
    Link(String, u32),
    SoundbitesList,
    LinkedSoundbitesList,
}

#[derive(Error, Debug, serde::Serialize)]
pub enum CommandError {
    #[error("Error locking mutex for command {0:?} [[{1}]]")]
    SenderMutex(Command, String),
    #[error("Error sending command {0:?} [[{1}]]")]
    Send(Command, String),
}

pub struct CommandState {
    pub sender: Mutex<Sender<Command>>,
}

impl CommandState {
    pub fn new(sender: Mutex<Sender<Command>>) -> CommandState {
        CommandState { sender }
    }
}
