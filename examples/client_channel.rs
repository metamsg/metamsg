use futures::{SinkExt, StreamExt};
use metamsg::string_codec::StringCodec;
use metamsg::Channel;
use std::io::Error;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time;

#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() {
    let string_codec = StringCodec::new();
    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let mut handles = Vec::new();
    for i in 1..8 {
        println!("channel-{} starting...", i);
        let handle = tokio::spawn(async move {
            let conn = TcpStream::connect(socket_addr).await.unwrap();
            let mut channel = Channel::new(conn, string_codec.clone());
            // Channel can't impl clone, send and recv must be a scope. The way is split conn to
            // make income channel and outcome channel.
            // tokio::spawn(async move {
            //     while let Some(v) = channel.next().await {
            //         println!("{}", v.unwrap());
            //     }
            // });
            println!("channel-{} started...", i);
            loop {
                time::sleep(Duration::from_secs(1)).await;
                let result = channel.send("hello".to_string()).await;
                match result {
                    Ok(_) => {}
                    Err(err) => {
                        println!("{:?}", err);
                        break;
                    }
                }
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        tokio::join!(handle);
    }
}
