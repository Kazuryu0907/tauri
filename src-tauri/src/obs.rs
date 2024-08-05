// use anyhow::{Ok,Result};
use obws::{client, Client};
use serde::{Deserialize,Serialize};
use tauri::Window;
// use core::time;
use std::{path::Path, thread::current, };
use time::Duration;
use obws::requests::custom::source_settings::SlideshowFile;
pub struct ObsConnection {
    pub host: String,
    pub port: u16,
    pub password: String,
}

#[derive(Deserialize,Debug)]
enum ColorRange {
    Auto = 0,
    Partial = 1,
    Full = 2,
}


const REPLAY_SOURCE_NAME: &str = "vlc_source";

pub struct ObsClass{
    pub client: Option<Client>
}

impl ObsClass{

    async fn get_client(&self) -> Result<&Client,String>{
        match &self.client{
            Some(client) => Ok(client),
            None => Err("Not connected to OBS".to_string())
        }
    }
    
    pub async fn connect(&mut self,data: &ObsConnection) -> Result<bool,String>{
        let client = Client::connect(data.host.clone(), data.port, Some(data.password.clone())).await;
        match client {
            Ok(client) => {self.client = Some(client);Ok(true)},
            Err(_) => Err("Failed to connect to OBS".to_string())
        }
    }

    pub async fn get_version(&self) -> Result<String,String>{
        let client = match &self.client{
            Some(client) => client,
            None => return Err("Not connected to OBS".to_string()),
        };
        let res_version = client.general().version().await;
        let version = match res_version{
            Ok(version) => version,
            Err(_) => return Err("Failed to get OBS version".to_string())
        };
        
        Ok(version.obs_version.to_string())
    }

    pub async fn is_exist_VLC_source(&self) -> Result<bool,String>{
        let client = self.get_client().await?;
        let res = client.inputs().list(Some(obws::requests::custom::source_settings::SOURCE_VLC_SOURCE)).await;
        match res {
            Ok(inputs) => {
                let mut iter = inputs.iter();
                let is_exist = iter.find(|&i| i.id.name == REPLAY_SOURCE_NAME);
                if is_exist.is_some() {
                    return Ok(true);
                } else {
                    return Ok(false);
                }
            },
            Err(_) => return Err("Failed to get VLC source".to_string())
        }
    }


    pub async fn init_VLC_source(&self) -> Result<(),String> {
        // もうあったら何もしない
        if self.is_exist_VLC_source().await? {
            return Ok(());
        }

        let client = self.get_client().await?;
        let current_scene = client.scenes().current_program_scene().await;
        let current_scene = match current_scene {
            Ok(current_scene) => current_scene,
            Err(_) => return Err("Failed to get current scene".to_string()),
        };

        let vlc_setting = obws::requests::custom::source_settings::VlcSource {
            loop_: false,
            shuffle: false,
            playback_behavior: obws::requests::custom::source_settings::PlaybackBehavior::StopRestart,
            playlist: &[],
            network_caching: Duration::milliseconds(100),
            track: 1,
            subtitle_enable:false,
            subtitle: 0,
        };
        let create = obws::requests::inputs::Create {
            scene: current_scene.id.into(),
            input: REPLAY_SOURCE_NAME,
            kind : obws::requests::custom::source_settings::SOURCE_VLC_SOURCE,
            settings : Some(vlc_setting),
            enabled: Some(false),
        };
        let res = client.inputs().create(create).await;
        match res {
            Ok(_) => println!("VLC source created"),
            Err(e) => println!("Failed to create VLC source: {}",e)
        }
        Ok(())
    }

    pub async fn play_vlc_source(&self,movie_pathes:&Vec<String>) -> Result<(),String>{
        let client = match &self.client{
            Some(client) => client,
            None => return Err("Not connected to OBS".to_string()),
        };

        if self.is_exist_VLC_source().await? {
            let input = obws::requests::inputs::InputId::Name(REPLAY_SOURCE_NAME);
            let res = client.inputs().remove(input).await;
            match res {
                Ok(_) => {},
                Err(e) => return Err(format!("Failed to delete VLC source: {}",e))
            }
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        let current_scene = client.scenes().current_program_scene().await;
        let current_scene = match current_scene {
            Ok(current_scene) => current_scene,
            Err(_) => return Err("Failed to get current scene".to_string()),
        };

        let mut playlists: Vec<SlideshowFile> = Vec::new();
        for movie_path in movie_pathes {
            let playlist = SlideshowFile {
                value: Path::new(movie_path),
                hidden: false,
                selected: false,
            };
            playlists.push(playlist);
        }
        let vlc_setting = obws::requests::custom::source_settings::VlcSource {
            loop_: false,
            shuffle: false,
            playback_behavior: obws::requests::custom::source_settings::PlaybackBehavior::StopRestart,
            playlist: &playlists, 
            network_caching: Duration::milliseconds(100),
            track: 1,
            subtitle_enable:false,
            subtitle: 0,
        };
        // let set_setting = obws::requests::inputs::SetSettings {
        //     input: obws::requests::inputs::InputId::Name(REPLAY_SOURCE_NAME),
        //     overlay: None,
        //     settings: &vlc_setting,
        // };
        let create = obws::requests::inputs::Create {
            scene: current_scene.id.into(),
            input: "vlc_source",
            kind : obws::requests::custom::source_settings::SOURCE_VLC_SOURCE,
            settings : Some(vlc_setting),
            enabled: Some(true),
        };
        let res = client.inputs().create(create).await;
        // let res = client.inputs().set_settings(set_setting).await;
        match res {
            Ok(_) => println!("VLC source created"),
            Err(e) => return Err(format!("Failed to create VLC source: {}",e))
        }
        
        Ok(())
    }

    pub async fn clip(&self,window:&Window) -> Result<(),String>{
        let res = self.capture_replay_buffer().await;
        if let Err(e) = res {
            println!("Failed to capture replay buffer: {}", e);
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        let file_name = self.get_replay_file_name().await?;
        println!("File name: {}", file_name);
        window.emit("capture_file", file_name).unwrap();
        Ok(())
    }
    pub async fn invoke_callback(&self) -> Result<String,String>{
        let client = match &self.client{
            Some(client) => client,
            None => return Err("Not connected to OBS".to_string()),
        };
        let res = client.replay_buffer().save().await;
        match res {
            Ok(_) => {},
            Err(_) => return Err("Failed to save replay buffer".to_string())
        }
        let file_name = client.replay_buffer().last_replay().await;
        match file_name {
            Ok(file_name) => Ok(file_name),
            Err(_) => return Err("Failed to get replay file name".to_string())
        }
        // client.scene_items().set_private_settings(settings)
    }

    pub async fn set_replay_buffer(&self) -> Result<(),String>{
        let client = match &self.client{
            Some(client) => client,
            None => return Err("Not connected to OBS".to_string()),
        };
        let replay_buffer_state = client.replay_buffer().status().await;
        let replay_buffer_state = match replay_buffer_state {
            Ok(replay_buffer_state) => replay_buffer_state,
            Err(_) => return Err("Failed to get replay buffer state".to_string())
        };
        // すでにonだった場合return
        if replay_buffer_state {
            return Ok(());
        }

        let res = client.replay_buffer().start().await;
        match res {
            Ok(_) => {},
            Err(_) => return Err("Failed to start replay buffer".to_string())
        }
        Ok(())
    }

    pub async fn capture_replay_buffer(&self) -> Result<(),String>{
        let client = match &self.client{
            Some(client) => client,
            None => return Err("Not connected to OBS".to_string()),
        };
        let res = client.replay_buffer().save().await;
        match res {
            Ok(_) => {},
            Err(_) => return Err("Failed to save replay buffer".to_string())
        }
        Ok(())
    }

    pub async fn get_replay_file_name(&self) -> Result<String,String>{
        let client = match &self.client{
            Some(client) => client,
            None => return Err("Not connected to OBS".to_string()),
        };
        let res = client.replay_buffer().last_replay().await;
        match res {
            Ok(file_name) => Ok(file_name),
            Err(_) => return Err("Failed to get replay file name".to_string())
        }
    }
}
