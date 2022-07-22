use metamsg::ServerBootstrap;
use metamsg::string_codec::StringCodec;

#[tokio::main(flavor = "multi_thread", worker_threads = 3)]
async fn main() {

    let string_codec = StringCodec::new();

    let server = ServerBootstrap::new(string_codec);

    server.start().await.unwrap();
}