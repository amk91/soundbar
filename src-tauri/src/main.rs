// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    path::PathBuf,
    fs,
    io::ErrorKind,
    sync::Mutex,
};

use simple_logging;
use log::LevelFilter;
use tauri::{self, Manager};
use crossbeam::channel::unbounded;
use directories::ProjectDirs;

mod app;
use app::commands::{self, utils::{Command, CommandState}};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }

            let (root_folder, logs_folder) = generate_app_folders();
            init_logging(&logs_folder);

            let (tx, rx) = unbounded::<Command>();

            app.manage(CommandState::new(Mutex::new(tx)));

            std::thread::spawn(|| {
                let mut app = app::App::new(root_folder, rx);
                app.run();
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_key,
            commands::add_soundbite,
            commands::set_soundbite_volume,
            commands::set_soundbite_speed,
            commands::link_soundbite_to_keytask,
            commands::get_soundbites_list,
            commands::get_linked_soundbites_list
        ])
        .run(tauri::generate_context!())
        .expect("unable to run tauri application");
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
    simple_logging::log_to_file(
        logs_folder.join("debug.txt"),
        LevelFilter::Debug
    ).expect("Unable to init logging on debug.txt");
    simple_logging::log_to_file(
        logs_folder.join("error.txt"),
        LevelFilter::Error
    ).expect("Unable to init logging on error.txt");
    simple_logging::log_to_file(
        logs_folder.join("info.txt"),
        LevelFilter::Info
    ).expect("Unable to init logging on info.txt");
}
