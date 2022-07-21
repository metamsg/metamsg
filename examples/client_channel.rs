use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use futures::SinkExt;
use tokio::net::TcpStream;
use metamsg::Channel;
use metamsg::string_codec::StringCodec;

#[tokio::main]
async fn main() {
    let string_codec = StringCodec::new();
    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let conn = TcpStream::connect(socket_addr).await.unwrap();
    let mut channel = Channel::new(conn, string_codec);
    loop {
        let _ = channel.send("hello".to_string()).await;
    }
}