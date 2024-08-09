// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod obs;
mod sound;
// mod key;


use futures_util::StreamExt;
use tauri::{App, Manager, Window};
use tauri_plugin_log::LogTarget;
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

use sound::Sound;


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
async fn obs_login_init(state: tauri::State<'_,Arc<TauriState>>,window:Window) -> Result<(),String> {
    let _obs_class = Arc::clone(&state);
    let obs_class = state.obs.lock().await;
    obs_class.set_replay_buffer().await?;
    obs_class.init_vlc_source().await?;
    let event_litener = obs_class.generate_event_listener().await?;
    // futures_util::pin_mut!(event_litener);
    let mut sound = Sound::new();
    let fut = event_litener.for_each(move |e| {
        match e {
            obws::events::Event::ReplayBufferSaved {path} => {
                // println!("Replay buffer saved: {:?}", path);
                window.emit("capture_file", path).unwrap();
                sound.play();
            },
            obws::events::Event::MediaInputPlaybackEnded { id } => {
                // PASS
            }
            _ => {
                println!("Event: {:?}", e);
            },
        }
        futures_util::future::ready(())
    });
    tokio::spawn(async move {fut.await});
    Ok(())
}


#[tauri::command]
async fn play_vlc_source(state: tauri::State<'_,Arc<TauriState>>,filenames: Vec<String>) -> Result<(),String> {
    let obs_class = state.obs.lock().await;
    obs_class.play_vlc_source(&filenames).await?;
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


async fn udp(state:Arc<TauriState>) -> Result<(),String>{
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
                // println!("Failed to parse json: {}", e);
                continue;
            }
        };
        if &json.cmd == "goals" || &json.cmd == "__goals__" {
            println!("Message: {}", msg);
            println!("Goals command received");
            let obs = state.obs.lock().await;
            if(&json.cmd == "goals"){sleep(tokio::time::Duration::from_millis(2000)).await;}
            // if(&json.cmd == "__goals__"){
            //     manager.play(sound_data.clone()).unwrap();
            // }
            let res = obs.capture_replay_buffer().await;
            if let Err(e) = res {
                println!("Failed to capture replay buffer: {}", e);
                continue;
            }
            // sleep(tokio::time::Duration::from_millis(100)).await;
            // let file_name = obs.get_replay_file_name().await;
            // let file_name = match file_name {
            //     Ok(file_name) => file_name,
            //     Err(e) => {println!("Failed to get replay file name: {}", e); continue;},
            // };
            // println!("File name: {}", file_name);
            // let res = obs.set_VLC_source().await;
            // match res {
            //     Ok(_) => {},
            //     Err(e) => println!("{}",e)
            // }
            // window.emit("capture_file", file_name).unwrap();
        }
    }
    
    Ok(())
}

use std::env;
use log::{error};
use std::{fs::File};
#[tokio::main]
async fn main() {
    // let log_file = File::create("log.txt").unwrap();
    // tracing_subscriber::fmt().with_max_level(tracing::Level::TRACE).with_writer(std::sync::Mutex::new(log_file)).finish();
    // tracing_subscriber::fmt().with_max_level(tracing::Level::TRACE).init();
    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());
    let tauri_state = Arc::new(TauriState{
        obs: Mutex::new(ObsClass{client: None})
    });
    let _tauri_state = Arc::clone(&tauri_state);
    // async fn setup_key(tauri_state:&Arc<TauriState>,window:&Window){
    //     let cl = ||{
    //         let _tauri_state = Arc::clone(tauri_state);
    //         let window = window.clone();
    //         tokio::spawn(async {
    //             let obs = tauri_state.obs.lock().await;
    //             obs.clip(&window).await;
    //         });
    //     };
    //     let f: key::CallbackFn = Box::new(cl);
    //     set_fn(f);
    
    // let _ = hook();
    


    tauri::Builder::default()
        .setup(move |app|{
            let _tauri_state = Arc::clone(&_tauri_state);
            // tokio::spawn(async {
            //     let res = hook();
            //     if let Err(e) = res {
            //         error!("Failed to hook: {}", e);
            //     }
            // });
            tokio::spawn(udp(_tauri_state));
            Ok(())
        })
        // .plugin(tauri_plugin_log::Builder::new()
        //     .targets([
        //         LogTarget::Stdout,
        //         LogTarget::Stderr,
        //         LogTarget::Webview,
        //         LogTarget::LogDir,
        //     ]).build(),
        // )
        .manage(Arc::clone(&tauri_state))
        .invoke_handler(tauri::generate_handler![get_obs_version,connect_to_obs,play_vlc_source,obs_login_init])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
