use metamsg::ServerBootstrap;
use metamsg::string_codec::StringCodec;

#[tokio::main]
async fn main() {

    let string_codec = StringCodec::new();

    let server = ServerBootstrap::new(string_codec);

    server.start().await.unwrap();
}