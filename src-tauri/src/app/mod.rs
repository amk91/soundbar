pub mod key_task;
pub mod soundbite;
mod key_hook;

use std::{
    path::PathBuf,
    fs,
    io::ErrorKind::NotFound,
    collections::HashMap,
};

use rodio::{OutputStreamHandle, OutputStream};
use directories::ProjectDirs;
use anyhow::{Result, bail};
use log::{error, debug, info};

use soundbite::Soundbite;
use key_task::KeyTask;
use key_hook::{KEY_TASK, init_key_hook};

//TODO: error handling
pub struct App {
    pub soundbites_dir: PathBuf,

    stream_handle: OutputStreamHandle,
    _stream: OutputStream,
    
    soundbites: HashMap<String, Soundbite>,
    soundtasks: HashMap<u16, String>,
}

impl App {
    pub fn new() -> App {
        init_key_hook();

        let soundbites_dir = if let Some(project_dirs) = ProjectDirs::from(
            "", "", "soundbar"
        ) {
            project_dirs.config_dir();
            PathBuf::from(project_dirs.data_dir())
        } else {
            panic!("Unable to generate project directories");
        };

        if let Err(err) = fs::metadata(soundbites_dir.as_path()) {
            if err.kind() == NotFound {
                if let Err(_) = fs::create_dir_all(soundbites_dir.as_path()) {
                    panic!(
                        "Unable to create directory {}",
                        soundbites_dir.display()
                    );
                }
            }
        }

        //TODO: give option to select a different output device
        let (_stream, stream_handle) = if let Ok(output) = OutputStream::try_default() {
            output
        } else {
            panic!("Unable to get default output stream");
        };

        App {
            soundbites_dir,
            stream_handle,
            _stream,
            soundbites: HashMap::new(),
            soundtasks: HashMap::new(),
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

            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }

    pub fn add_soundbite(
        &mut self,
        name: String,
        path: &PathBuf
    ) -> Result<()> {
        if let Ok(soundbite) = Soundbite::new(&self.stream_handle, &path) {
            info!("Soundbite {} generated, filepath {:?}", name, path);
            self.soundbites.insert(name, soundbite);
            return Ok(());
        } else {
            error!("{} - {:?}", name, path);
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
            error!("Invalid key combination, {} - {:?}", soundbite_name, key_task);
            bail!("Key combination is invalid or empty");
        } else {
            error!("Unable to find soundbite {}", soundbite_name);
            bail!("Unable to find soundbite {}", soundbite_name);
        }
    }

    pub fn play_soundbite(&self, soundbite_name: &String) -> Result<()> {
        if let Some(soundbite) = &self.soundbites.get(soundbite_name) {
            soundbite.play();
            Ok(())
        } else {
            error!("Unable to find soundbite {}", soundbite_name);
            bail!("Unable to find soundbite {}", soundbite_name);
        }
    }
}
