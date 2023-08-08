// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    sync::{Arc, Mutex},
    path::{PathBuf, Path},
    fs::{self, File},
    io::{ErrorKind, BufReader, BufRead, BufWriter, prelude::*},
    thread,
};

use directories::ProjectDirs;
use simple_logging;
use log::{trace, error, LevelFilter};
use tauri::{self, State, Manager};
use crossbeam::channel::unbounded;

mod soundmanager;
use soundmanager::{
    SoundManager,
    soundstate::SoundState,
    Soundbites,
    soundbite::{
        Soundbite,
        SoundbiteData,
    },
    SoundbitesKeyTasks,
    SOUNDBITES_FILE,
    KEYTASKS_FILE,
    ROOT_FOLDER, key_task::KeyTaskCode,
};

mod commands;
use commands::*;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }

            let (root_folder, logs_folder) = generate_app_folders();
            if let Ok(folder) = ROOT_FOLDER.lock().as_mut() {
                folder.clone_from(&root_folder);
            }

            init_logging(&logs_folder);

            trace!("App init");
            trace!("Root folder: {}", root_folder.display());
            trace!("Logs folder: {}", logs_folder.display());

            let soundbites = Arc::new(Mutex::new(load_soundbites()));
            let soundbites_keytasks = Arc::new(Mutex::new(load_keytasks()));

            let (messages_tx, messages_rx) = unbounded();
            let (responses_tx, responses_rx) = unbounded();

            app.manage(SoundState::new(
                soundbites.clone(),
                soundbites_keytasks.clone(),
                messages_tx,
                responses_rx,
            ));

            thread::spawn(move || {
                SoundManager::new(
                    messages_rx,
                    responses_tx,
                    soundbites,
                    soundbites_keytasks
                ).run();
            });

            Ok(())
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { .. } => {
                let state: State<SoundState> = event.window().state();
                save_soundbites(&state);
                save_keytasks(&state);
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            add_soundbite,
            remove_soundbite,
            play_soundbite,
            stop_soundbite,
            set_name,
            set_volume,
            set_speed,
            set_keytask_code,
            remove_keytask_code,
            get_soundbite,
            get_soundbites,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn generate_app_folders() -> (PathBuf, PathBuf) {
    let root_folder = if let Some(project_dirs) = ProjectDirs::from(
        "", "", "soundbar"
    ) {
        if let Some(folder) = project_dirs.config_dir().parent() {
            PathBuf::from(folder)
        } else {
            panic!("Unable to generate parent project directories")
        }
    } else {
        panic!("Unable to generate project directories");
    };

    let generate_folder = |folder: &PathBuf| {
        if let Err(err) = fs::metadata(folder.as_path()) {
            if err.kind() == ErrorKind::NotFound {
                if let Err(_) = fs::create_dir_all(folder.as_path()) {
                    panic!(
                        "Unable to create directory {}",
                        folder.display()
                    );
                }
            }
        }
    };

    generate_folder(&root_folder);

    let logs_folder = root_folder.clone().join("logs");
    generate_folder(&logs_folder);

    (root_folder, logs_folder)
}

fn init_logging(logs_folder: &PathBuf) {
    if cfg!(debug_assertions) {
        if let Err(err) = fs::metadata(Path::new("logs\\")) {
            if err.kind() == ErrorKind::NotFound {
                if let Err(_) = fs::create_dir_all(Path::new("logs\\")) {
                    panic!("Unable to create directory logs\\ in debug");
                }
            }
        }

        simple_logging::log_to_file(
            "logs\\trace.txt",
            LevelFilter::Trace,
        ).expect("Unable to init logging for trace on info.txt in debug");
        simple_logging::log_to_file(
            "logs\\trace.txt",
            LevelFilter::Error,
        ).expect("Unable to init logging for error on info.txt in debug");
    } else {
        simple_logging::log_to_file(
            logs_folder.join("trace.txt"),
            LevelFilter::Trace
        ).expect("Unable to init logging for trace on info.txt");
        simple_logging::log_to_file(
            logs_folder.join("trace.txt"),
            LevelFilter::Error
        ).expect("Unable to init logging for error on info.txt");
    }
}

fn load_soundbites() -> Soundbites {
    let mut soundbites = Soundbites::new();
    if let Ok(root_folder) = ROOT_FOLDER.lock().as_ref() {
        if let Ok(file) = File::open(root_folder.join(SOUNDBITES_FILE)) {
            let reader = BufReader::new(file);
    
            for line in reader.lines() {
                if let Ok(line) = line {
                    if let Ok(soundbite_data) = serde_json::from_str::<SoundbiteData>(&line) {
                        soundbites.push(Soundbite::from_data(soundbite_data));
                    }
                }
            }
        }

        
    }

    soundbites
}

fn load_keytasks() -> SoundbitesKeyTasks {
    let mut keytasks = SoundbitesKeyTasks::new();

    if let Ok(root_folder) = ROOT_FOLDER.lock().as_ref() {
        if let Ok(file) = File::open(root_folder.join(KEYTASKS_FILE)) {
            let reader = BufReader::new(file);

            for line in reader.lines() {
                if let Ok(line) = line {
                    if let Ok(keytask) = serde_json::from_str::<(KeyTaskCode, usize)>(&line) {
                        keytasks.insert(keytask.0, keytask.1);
                    }
                }
            }
        }
    }

    keytasks
}

fn save_soundbites(state: &State<SoundState>) {
    let soundbites = state.soundbites.lock().unwrap();
    
    if let Ok(root_folder) = ROOT_FOLDER.lock().as_ref() {
        let file = File::create(root_folder.join(SOUNDBITES_FILE)).unwrap();
        let mut writer = BufWriter::new(file);
    
        for soundbite in soundbites.iter() {
            match serde_json::to_string(&soundbite.data) {
                Ok(string) => {
                    writer.write(string.as_bytes()).map_err(|err|
                        error!(
                            "Unable to save soundbite named {} [[{:?}]]",
                            soundbite.data.name,
                            err
                        )
                    ).unwrap();
                },
                Err(err) => error!(
                    "Unable to write soundbite named {} on file [[{:?}]]",
                    soundbite.data.name,
                    err
                ),
            }
        }
    
        if let Err(err) = writer.flush() {
            error!("Unable to flush buffer to save soundbites [[{:?}]]", err);
        }
    }
}

fn save_keytasks(state: &State<SoundState>) {
    let soundbites_keytasks = state.soundbites_keytasks.lock().unwrap();

    if let Ok(root_folder) = ROOT_FOLDER.lock().as_ref() {
        let file = File::create(root_folder.join(KEYTASKS_FILE)).unwrap();
        let mut writer = BufWriter::new(file);

        for keytask in soundbites_keytasks.iter() {
            match serde_json::to_string(&keytask) {
                Ok(string) => {
                    writer.write(string.as_bytes()).map_err(|err|
                        error!(
                            "Unable to save keytask {} - {} [[{:?}]]",
                            keytask.0,
                            keytask.1,
                            err
                        )
                    ).unwrap();
                },
                Err(err) => error!(
                    "Unable to write keytask {} - {} on file [[{:?}]]",
                    keytask.0,
                    keytask.1,
                    err
                ),
            }
        }
    }
}
