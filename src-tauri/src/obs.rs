// use anyhow::{Ok,Result};
use obws::{Client};
use serde::{Deserialize,Serialize};
// use core::time;
use std::{path::Path};
use time::Duration;
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


pub struct ObsClass{
    pub client: Option<Client>
}

impl ObsClass{
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

    pub async fn set_VLC_source(&self) -> Result<(),String>{
        let client = match &self.client{
            Some(client) => client,
            None => return Err("Not connected to OBS".to_string()),
        };
        let playlist = obws::requests::custom::source_settings::SlideshowFile {
            value: Path::new("C:\\Users\\kazum\\Videos\\2024-05-06 23-13-27.mp4"),
            hidden: false,
            selected: false,
        };
        let playlist2 = obws::requests::custom::source_settings::SlideshowFile {
            value: Path::new("C:\\Users\\kazum\\Videos\\Replay 2024-08-01 01-14-25.mp4"),
            hidden: false,
            selected: false,
        };
        let vlc_setting = obws::requests::custom::source_settings::VlcSource {
            loop_: false,
            shuffle: false,
            playback_behavior: obws::requests::custom::source_settings::PlaybackBehavior::StopRestart,
            playlist: &[playlist,playlist2],
            network_caching: Duration::milliseconds(100),
            track: 1,
            subtitle_enable:false,
            subtitle: 0,
        };
        // let setting: Result<obws::responses::inputs::InputSettings<FfmpegSource>, obws::Error> = client.inputs().settings(obws::requests::inputs::InputId::Name("メディアソース")).await;
        // let setting: Result<FfmpegSource, obws::Error> = client.inputs().default_settings(obws::requests::custom::source_settings::SOURCE_FFMPEG_SOURCE).await;
        // match setting {
        //     Ok(setting) => {
        //         println!("{:?}",setting);
        //     },
        //     Err(e) => return Err(format!("Failed to get VLC source settings: {}",e))
        // }
        // let create = obws::requests::inputs::Create {
        //     scene: obws::requests::scenes::SceneId::Name("Replayiiiiiiiiiii"),
        //     input: "kind",
        //     kind : obws::requests::custom::source_settings::SOURCE_VLC_SOURCE,
        //     settings : Some(vlc_setting),
        //     enabled: Some(true),
        // };
        let create = obws::requests::inputs::Create {
            scene: obws::requests::scenes::SceneId::Name("VALO"),
            input: "vlc_source",
            kind : obws::requests::custom::source_settings::SOURCE_VLC_SOURCE,
            settings : Some(vlc_setting),
            enabled: Some(true),
        };
        let res = client.inputs().create(create).await;
        match res {
            Ok(_) => println!("VLC source created"),
            Err(e) => return Err(format!("Failed to create VLC source: {}",e))
        }
        
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
