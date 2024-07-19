// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod obs;
use std::{borrow::BorrowMut, sync::Mutex};

use obs::ObsClass;
use tauri::Manager;
use tokio;


struct TauriState{
    obs: Mutex<ObsClass>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn connect_to_obs(state: tauri::State<TauriState>,host:String, port:u16, password:String) -> String {
    let obs_connection = obs::ObsConnection {
        host: host,
        port: port,
        password: password,
    };
    let mut obs_class = state.obs.lock().unwrap();
    let connect = async {
        obs_class.connect(&obs_connection).await
    };
    let rt = tokio::runtime::Runtime::new().unwrap();
    let cli = rt.block_on(connect);
    match cli{
        Ok(_) =>  format!("Connected to OBS"),
        Err(_) => format!("Failed to connect to OBS")
    }
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,connect_to_obs])
        .manage(TauriState{
            obs: Mutex::new(obs::ObsClass{client: None})
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
