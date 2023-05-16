// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;

use std::{
    path::Path,
    fs::create_dir,
    env::current_dir,
};

use simple_logging;
use log::LevelFilter;
use tauri::{self, Manager};

//TODO: make tauri::commands return results

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello {}!", name)
}

#[tauri::command]
fn add_soundbite(path: String) {
    panic!("add_soundbite not implemented");
}

#[tauri::command]
fn set_soundbite_volume(soundbite_name: String, volume: f32) {
    panic!("set_soundbite_volume not implemented")
}

#[tauri::command]
fn set_soundbite_speed(soundbite_name: String, speed: f32) {
    panic!("set_soundbite_speed not implemented")
}

#[tauri::command]
fn link_soundbite_to_keytask(key_code: u32, soundbite_name: String) {
    panic!("link_soundbite_to_keytask not implemented");
}

#[tauri::command]
fn get_soundbites_list() -> Vec<String> {
    panic!("get_soundbites_list not implemented");
}

#[tauri::command]
fn get_linked_soundbites_list() -> Vec<(u32, String)> {
    panic!("get_linked_soundbites_list not implemented");
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String
}

fn main() {
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

    //TODO: generate logs folder
    simple_logging::log_to_file(
        "logs/debug.txt",
        LevelFilter::Debug
    ).expect("Unable to init logging on debug.txt");
    simple_logging::log_to_file(
        "logs/error.txt",
        LevelFilter::Error
    ).expect("Unable to init logging on error.txt");
    simple_logging::log_to_file(
        "logs/info.txt",
        LevelFilter::Info
    ).expect("Unable to init logging on info.txt");

    std::thread::spawn(|| {
        let mut app = app::App::new();
        app.run();
    });

    tauri::Builder::default()
        .setup(|app| {
            let id = app.listen_global("event-name", |event| {
                println!("got event-name with payload {:?}", event.payload());
            });
            app.unlisten(id);

            app.emit_all("event-name", Payload { message: "Tauri is awesome".into() })?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            add_soundbite,
            set_soundbite_volume,
            set_soundbite_speed,
            link_soundbite_to_keytask,
            get_soundbites_list,
            get_linked_soundbites_list
        ])
        .run(tauri::generate_context!())
        .expect("unable to run tauri application");
}
