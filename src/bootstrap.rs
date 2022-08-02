use crate::channel::Channel;
use crate::BoxError;
use futures::StreamExt;
use std::fmt::Debug;
use std::io;
use std::marker::PhantomData;
use std::net::SocketAddr;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpListener;
use tokio_util::codec::{Decoder, Encoder};

/// ServerBootstrap create a listener and bind port, then accept connection and create a `Channel`.
///
/// ServerBootstrap need an transport and one codec, maybe also need a Proto.
#[derive(Debug)]
pub struct ServerBootstrap<Codec, Item> {
    codec: Codec,
    _phantom: PhantomData<Item>,
}

impl<Codec, Item> ServerBootstrap<Codec, Item>
where
    Codec: Debug + Clone + Encoder<Item> + Decoder + Send + Sync + 'static,
    <Codec as Encoder<Item>>::Error: From<io::Error> + Debug,
    <Codec as Decoder>::Error: From<io::Error> + Debug,
    Item: Debug + Send + 'static,
    <Codec as Decoder>::Item: Debug,
{
    /// Create a ServerBootstrap with codec.
    pub fn new(codec: Codec) -> Self {
        Self {
            codec,
            _phantom: PhantomData::default(),
        }
    }

    /// Start a Server.
    pub async fn start(&self) -> Result<(), BoxError> {
        let listener = TcpListener::bind("127.0.0.1:8080").await?;

        loop {
            let (conn, remote) = listener.accept().await?;
            let codec = self.codec.clone();
            tokio::spawn(async move {
                init_channel(conn, codec, remote).await;
            });
        }
    }
}

/// Create a `Channel` when a connection come on, and then create a handle chain.
async fn init_channel<Conn, Codec, Item>(socket: Conn, codec: Codec, remote: SocketAddr)
where
    Conn: AsyncRead + AsyncWrite + Send + Unpin + 'static + Debug,
    Codec: Debug + Clone + Encoder<Item> + Decoder + Send + Sync + 'static,
    <Codec as Encoder<Item>>::Error: From<io::Error> + Debug,
    <Codec as Decoder>::Error: From<io::Error> + Debug,
    Item: Debug + Send + 'static,
    <Codec as Decoder>::Item: Debug,
{
    println!("a new conn come in");
    let mut channel = Channel::new(socket, codec);
    println!("{:?}", channel);
    while let Some(v) = channel.next().await {
        // There‘s no know item's type, in general, send is called in handle, that time, item's type
        // maybe has been actual.
        // let _ = channel.send("world");
        println!("{:?}, {:?}", remote, v.unwrap());
    }
}
