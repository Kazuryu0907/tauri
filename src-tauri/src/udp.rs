use tokio::net::UdpSocket;
use serde::{Deserialize,Serialize};
#[derive(Serialize,Deserialize,Debug)]
struct CommandData{
    cmd: String,
    data: Option<String>,
}

pub async fn udp<F,Fut>(invokeCallback:F) -> std::io::Result<()>
where F: Fn() -> Fut,
      Fut: std::future::Future<Output = (String)>
{
    let socket = UdpSocket::bind("127.0.0.1:12345").await;
    let socket = match socket {
        Ok(socket) => socket,
        Err(e) => {
            println!("Failed to bind socket: {}", e);
            return Err(e);
        }
    };
    let mut buf = [0;2048];

    loop {
        let res = socket.recv_from(&mut buf).await;
        let (len, arr) = match res {
            Ok((len, arr)) => (len, arr),
            Err(e) => {
                println!("Failed to receive from socket: {}", e);
                continue;
            }
        };
        println!("Received {} bytes from {}", len, arr);
        let msg = std::str::from_utf8(&buf[..len]).unwrap();
        println!("Message: {}", msg);
        let json = string_to_json(msg);
        dbg!(&json);
        if json.cmd == "goals" {
            invokeCallback();
        }
    }
}

fn string_to_json(msg: &str) -> CommandData {
    let json: CommandData = serde_json::from_str(msg).unwrap();
    return json;
}