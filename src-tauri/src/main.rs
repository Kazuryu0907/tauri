// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod obs;
mod udp;

// use std::fmt::Result;
use anyhow::{Result,Error};

use obs::ObsClass;
use tokio::sync::Mutex;


struct TauriState{
    obs: Mutex<ObsClass>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_obs_version(state: tauri::State<'_,TauriState>) -> Result<String,String> {
    let obs_class = state.obs.lock().await;
    let res = obs_class.get_version().await;
    match res{
        Ok(version) => Ok(version),
        Err(_) => Err("Failed to get OBS version".to_string())
    }
}

#[tauri::command]
async fn setup_replay_buffer(state: tauri::State<'_,TauriState>) -> Result<(),String> {
    let obs_class = state.obs.lock().await;
    let res = obs_class.set_replay_buffer().await;
    match res {
        Ok(_) => Ok(()),
        Err(_) => return Err("Failed to start replay buffer".to_string())
    }
}

#[tauri::command]
async fn setup_udp(state: tauri::State<'_,TauriState>) -> Result<(),String>{
    let obs_class = state.obs.lock().await;
    udp::udp(obs_class.invoke_callback);
    Ok(())

}

#[tauri::command]
async fn connect_to_obs(state: tauri::State<'_,TauriState>,host:String, port:u16, password:String) -> Result<String,String> {
    let obs_connection = obs::ObsConnection {
        host: host,
        port: port,
        password: password,
    };
    let mut obs_class = state.obs.lock().await;
    let res = obs_class.connect(&obs_connection).await;
    match res{
        Ok(_) =>  Ok("Connected to OBS".to_string()),
        Err(_) => Err("Failed to connect to OBS".to_string())
    }
}



#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_obs_version,connect_to_obs])
        .manage(TauriState{
            obs: Mutex::new(obs::ObsClass{client: None})
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
