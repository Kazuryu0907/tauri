use obws::Client;

pub struct ObsConnection {
    pub host: String,
    pub port: u16,
    pub password: String,
}

pub async fn obs(data: ObsConnection) -> Result<Client,&'static str>{
    let ObsConnection {host,port,password} = data;
    let option_password = Some(password);
    let client = Client::connect(host, port, option_password).await;
    match client {
        Ok(client) => return Ok(client),
        Err(_) => return Err("Failed to connect to OBS")
    }
    // client
}