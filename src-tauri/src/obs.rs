use obws::responses::inputs::InputId;
use obws::responses::scene_items::SceneItem;
use obws::Client;
use serde::Deserialize;
// use core::time;
use std::path::Path;
use time::Duration;
use obws::requests::custom::source_settings::SlideshowFile;
use obws::events::Event;
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

    pub async fn is_exist_vlc_source(&self) -> Result<bool,String>{
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


    pub async fn init_vlc_source(&self) -> Result<(),String> {
        // もうあったら何もしない
        if self.is_exist_vlc_source().await? {
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


    async fn get_current_scene(&self) -> Result<obws::responses::scenes::CurrentProgramScene,String> {
        let client = self.get_client().await?;
        let current_scene = client.scenes().current_program_scene().await;
        match current_scene {
            Ok(current_scene) => Ok(current_scene),
            Err(_) => Err("Failed to get current scene".to_string())
        }
    }

    async fn get_replay_source_item_id(&self) -> Result<i64,String> {
        let client = self.get_client().await?;
        let current_scene = self.get_current_scene().await?;
        let scene_items = client.scene_items().list(current_scene.id.clone().into()).await;
        let scene_items = match scene_items {
            Ok(scene_items) => scene_items,
            Err(_) => return Err("Failed to get scene items".to_string())
        };
        let replay_source_item: Vec<&SceneItem> = scene_items.iter().filter(|&item| {
            item.source_name == REPLAY_SOURCE_NAME
        }).collect();
        Ok(replay_source_item[0].id)
    }

    pub async fn play_vlc_source(&self,movie_pathes:&Vec<String>) -> Result<(),String>{
        let client = match &self.client{
            Some(client) => client,
            None => return Err("Not connected to OBS".to_string()),
        };

        // if self.is_exist_vlc_source().await? {
        //     let input = obws::requests::inputs::InputId::Name(REPLAY_SOURCE_NAME);
        //     let res = client.inputs().remove(input).await;
        //     match res {
        //         Ok(_) => {},
        //         Err(e) => return Err(format!("Failed to delete VLC source: {}",e))
        //     }
        // }
        // tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        let current_scene = self.get_current_scene().await?; 

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
        let set_setting = obws::requests::inputs::SetSettings {
            input: obws::requests::inputs::InputId::Name(REPLAY_SOURCE_NAME),
            overlay: Some(true),
            settings: &vlc_setting,
        };
        // let create = obws::requests::inputs::Create {
        //     scene: current_scene.id.into(),
        //     input: "vlc_source",
        //     kind : obws::requests::custom::source_settings::SOURCE_VLC_SOURCE,
        //     settings : Some(vlc_setting),
        //     enabled: Some(true),
        // };
        // let res = client.inputs().create(create).await;
        let res = client.inputs().set_settings(set_setting).await;
        match res {
            Ok(_) => println!("VLC source updated"),
            Err(e) => return Err(format!("Failed to create VLC source: {}",e))
        }
        let scene_items = client.scene_items().list(current_scene.id.clone().into()).await;
        let scene_items = match scene_items {
            Ok(scene_items) => scene_items,
            Err(_) => return Err("Failed to get scene items".to_string())
        };
        let replay_source_item: Vec<&SceneItem> = scene_items.iter().filter(|&item| {
            item.source_name == REPLAY_SOURCE_NAME
        }).collect();
        let set_enabled = obws::requests::scene_items::SetEnabled {
            scene: current_scene.id.into(),
            item_id: replay_source_item[0].id,
            enabled: true,
        };
        let res = client.scene_items().set_enabled(set_enabled).await;
        if let Err(e) = res {
            return Err(format!("Failed to set VLC source enabled: {}",e));
        }
        
        Ok(())
    }

    pub async fn on_playback_ended(&self,id:InputId) -> Result<(),String> {
        let client = self.get_client().await?;
        if id.name == REPLAY_SOURCE_NAME {
            let current_scene = self.get_current_scene().await?;
            let item_id = self.get_replay_source_item_id().await?;
            let set_enabled = obws::requests::scene_items::SetEnabled {
                scene: current_scene.id.into(),
                item_id: item_id,
                enabled: false,
            };
            let res = client.scene_items().set_enabled(set_enabled).await;
            if let Err(e) = res {
                return Err(format!("Failed to set VLC source disabled: {}",e));
            }
        }
        Ok(())
    }

    pub async fn generate_event_listener(&self) -> Result<impl futures_util::Stream<Item = Event>,String> {
        let client = self.get_client().await?;
        let events = client.events();
        match events {
            Ok(events) => Ok(events),
            Err(_) => return Err("Failed to generate events listener".to_string())
        }
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

}
