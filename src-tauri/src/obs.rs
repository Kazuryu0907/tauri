use obws::{Client};

pub struct ObsConnection {
    pub host: String,
    pub port: u16,
    pub password: String,
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

    pub async fn set_replay_buffer(&self) -> Result<(),String>{
        let client = match &self.client{
            Some(client) => client,
            None => return Err("Not connected to OBS".to_string()),
        };
        let res = client.replay_buffer().start().await;
        match res {
            Ok(_) => {},
            Err(_) => return Err("Failed to start replay buffer".to_string())
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
