// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod obs;
use tokio;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn connect_to_obs(host:String, port:u16, password:String) -> String {
    let obs_connection = obs::ObsConnection {
        host: host,
        port: port,
        password: password,
    };
    let obs_client = async {
        obs::obs(obs_connection).await
    };
    let rt = tokio::runtime::Runtime::new().unwrap();
    let cli = rt.block_on(obs_client);
    match cli{
        Ok(_) => return format!("Connected to OBS"),
        Err(_) => return format!("Failed to connect to OBS")
    }
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,connect_to_obs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
