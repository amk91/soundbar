// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    sync::{Arc, Mutex},
    path::PathBuf,
    fs,
    io::ErrorKind,
    thread,
};

use directories::ProjectDirs;
use simple_logging;
use log::LevelFilter;
use tauri::{self, Manager};
use crossbeam::channel::unbounded;

mod soundmanager;
use soundmanager::{
    SoundManager,
    soundstate::SoundState,
    Soundbites,
    SoundbitesKeyTasks,
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
            init_logging(&logs_folder);

            let soundbites = Arc::new(Mutex::new(Soundbites::new()));
            let soundbites_keytasks = Arc::new(Mutex::new(SoundbitesKeyTasks::new()));

            let (new_soundbite_tx, new_soundbite_rx) = unbounded();
            let (new_soundbite_ack_tx, new_soundbite_ack_rx) = unbounded();

            app.manage(SoundState::new(
                soundbites.clone(),
                soundbites_keytasks.clone(),
                new_soundbite_tx,
                new_soundbite_ack_rx,
            ));

            thread::spawn(move || {
                SoundManager::new(
                    root_folder,
                    new_soundbite_rx,
                    new_soundbite_ack_tx,
                    soundbites.clone(),
                    soundbites_keytasks.clone()
                ).run();
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            add_soundbite,
            remove_soundbite,
            set_volume,
            set_speed,
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
