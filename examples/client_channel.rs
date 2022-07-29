use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::thread;
use std::time::Duration;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use metamsg::Channel;
use metamsg::string_codec::StringCodec;

#[tokio::main(flavor = "multi_thread", worker_threads = 3)]
async fn main() {
    let string_codec = StringCodec::new();
    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let mut handles = Vec::new();
    for i in 1..3 {
        println!("channel-{} starting...", i);
        let handle = tokio::spawn( async move {
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
                thread::sleep(Duration::from_secs(1));
                let _ = channel.send("hello".to_string()).await;
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        tokio::join!(handle);
    }
}