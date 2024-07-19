use obws::{client, Client};

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
        let version = client.general().version().await?;
    }
}
