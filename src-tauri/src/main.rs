// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod obs;

use tauri::{App, Manager, Window};
use tokio::net::UdpSocket;

use tokio::time::{Duration, sleep};

// use std::fmt::Result;
use anyhow::{Result,Error};

use obs::ObsClass;
use tokio::sync::Mutex;
use std::{iter::Map, sync::Arc};

use sonic_rs::{Deserialize, Object, Serialize};

#[derive(Debug,Serialize,Deserialize)]
struct CommandData{
    cmd: String,
    data: Option<Object>,
}

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
async fn setup_replay_buffer(state: tauri::State<'_,Arc<TauriState>>) -> Result<(),String> {
    let obs_class = state.obs.lock().await;
    let res = obs_class.set_replay_buffer().await;
    match res {
        Ok(_) => Ok(()),
        Err(_) => return Err("Failed to start replay buffer".to_string())
    }
}

#[tauri::command]
async fn save_replay_buffer(state: tauri::State<'_,TauriState>) -> Result<String,String> {
    let obs_class = state.obs.lock().await;
    let res = obs_class.invoke_callback().await;
    match res {
        Ok(file_name) => Ok(file_name),
        Err(_) => return Err("Failed to save replay buffer".to_string())
    }
}


#[tauri::command]
async fn setup_udp(state: tauri::State<'_,TauriState>) -> Result<(),String>{
    let obs_class = state.obs.lock().await;
    // udp::udp(obs_class.invoke_callback);
    Ok(())

}

#[tauri::command]
async fn connect_to_obs(state: tauri::State<'_,Arc<TauriState>>,host:String, port:u16, password:String) -> Result<String,String> {
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


async fn udp(state:Arc<TauriState>, window:Window) -> Result<(),String>{
    let socket = UdpSocket::bind("0.0.0.0:12345").await;
    let socket = match socket {
        Ok(socket) => socket,
        Err(e) => {
            println!("Failed to bind socket: {}", e);
            return Err(e.to_string());
        }
    };
    let mut buf = [0u8;2048];

    loop {
        let res = socket.recv_from(&mut buf).await;
        let (len, arr) = match res {
            Ok((len, arr)) => (len, arr),
            Err(e) => {
                println!("Failed to receive from socket: {}", e);
                continue;
            }
        };
        // println!("Received {} bytes from {}", len, arr);
        let msg = std::str::from_utf8(&buf[..len]).unwrap();
        let json: std::result::Result<CommandData, sonic_rs::Error> = sonic_rs::from_str(msg);
        let json = match json {
            Ok(json) => json,
            Err(e) => {
                println!("Failed to parse json: {}", e);
                continue;
            }
        };
        if &json.cmd == "goals" {
            println!("Message: {}", msg);
            println!("Goals command received");
            let obs = state.obs.lock().await;
            let res = obs.capture_replay_buffer().await;
            if let Err(e) = res {
                return Err(format!("Failed to capture replay buffer: {}", e));
            }
            sleep(Duration::from_secs(1)).await;
            let file_name = obs.get_replay_file_name().await;
            let file_name = match file_name {
                Ok(file_name) => file_name,
                Err(e) => return Err(format!("Failed to get replay file name: {}", e)),
            };
            println!("File name: {}", file_name);
            let res = obs.set_VLC_source().await;
            match res {
                Ok(_) => {},
                Err(e) => println!("{}",e)
            }
            window.emit("capture_file", file_name).unwrap();
        }
    }
    
    Ok(())
}

#[tokio::main]
async fn main() {
    // tracing_subscriber::fmt().with_max_level(tracing::Level::TRACE).init();
    let tauri_state = Arc::new(TauriState{
        obs: Mutex::new(ObsClass{client: None})
    });
    let _tauri_state = Arc::clone(&tauri_state);
    tauri::Builder::default()
        .setup(move |app|{
            let _tauri_state = Arc::clone(&_tauri_state);
            let main_window = app.get_window("main").unwrap();
            
            tokio::spawn(udp(_tauri_state,main_window));
            Ok(())
        })
        .manage(Arc::clone(&tauri_state))
        .invoke_handler(tauri::generate_handler![get_obs_version,connect_to_obs,setup_replay_buffer])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
