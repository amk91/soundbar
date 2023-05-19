// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    path::Path,
    fs::create_dir,
    env::current_dir, sync::Mutex,
};

use simple_logging;
use log::LevelFilter;
use tauri::{self, Manager};
use crossbeam::channel::unbounded;

mod app;
use app::commands::{self, utils::{Command, CommandState}};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }

            //TODO: generate logs folder
            match current_dir().as_mut() {
                Ok(dir) => {
                    let logs_dir = Path::new(&dir).join("logs");
                    if !logs_dir.is_dir() {
                        if let Err(err) = create_dir(logs_dir) {
                            panic!("{}", err);
                        }
                    }
                },
                Err(err) => panic!("{}", err),
            }

            // simple_logging::log_to_file(
            //     "logs/debug.txt",
            //     LevelFilter::Debug
            // ).expect("Unable to init logging on debug.txt");
            // simple_logging::log_to_file(
            //     "logs/error.txt",
            //     LevelFilter::Error
            // ).expect("Unable to init logging on error.txt");
            // simple_logging::log_to_file(
            //     "logs/info.txt",
            //     LevelFilter::Info
            // ).expect("Unable to init logging on info.txt");
        
            let (tx, rx) = unbounded::<Command>();

            app.manage(CommandState::new(Mutex::new(tx)));

            std::thread::spawn(|| {
                let mut app = app::App::new(rx);
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
