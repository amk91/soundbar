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

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CommandPayload {
    pub soundbites_list: Vec<String>,
    pub linked_soundbites_list: Vec<(String, String)>,
}

#[derive(Error, Debug, Clone, serde::Serialize)]
pub enum CommandError {
    #[error("Unrecognized command")]
    UnrecognizedCommand,
    
    #[error("Error locking mutex for command {0:?} [[{1}]]")]
    SenderMutex(Command, String),
    #[error("Error sending command {0:?} [[{1}]]")]
    Send(Command, String),
    
    #[error("Soundbite name {0} already used")]
    SoundbiteNameUsed(String),
    #[error("Unable to generate soundbite from file {0}")]
    SoundbiteGenerationFailed(String),
    #[error("Soundbite {0} not found")]
    SoundbiteNotFound(String),

    #[error("Key {0} already assigned to soundbite {1}")]
    KeyAlreadyAssigned(String, String),
    #[error("Invalid key combination")]
    KeyCombinationInvalid,
}

pub type CommandResult = anyhow::Result<CommandPayload, CommandError>;

pub struct CommandState {
    pub sender: Mutex<Sender<Command>>,
}

impl CommandState {
    pub fn new(sender: Mutex<Sender<Command>>) -> CommandState {
        CommandState { sender }
    }
}
