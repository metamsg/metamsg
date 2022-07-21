use std::fmt::Debug;
use std::marker::PhantomData;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::{TcpListener};
use tokio_stream::StreamExt;
use tokio_util::codec::{Decoder, Encoder};
use crate::BoxError;
use crate::channel::Channel;

/// ServerBootstrap create a listener and bind port, then accept connection and create a `Channel`.
///
/// ServerBootstrap need an transport and one codec, maybe also need a Proto.
#[derive(Debug)]
pub struct ServerBootstrap<Codec, Item> {
   codec: Codec,
   _phantom: PhantomData<Item>,
}

impl <Codec, Item> ServerBootstrap<Codec, Item>
where
    Codec: Debug + Clone + Encoder<Item> + Decoder + Send + 'static,
    Item: Debug + Send + 'static
{
   /// Create a ServerBootstrap with codec.
   pub fn new(codec: Codec) -> Self {
      Self {
         codec,
         _phantom: PhantomData::default(),
      }
   }

   /// Start a Server.
   pub async fn start(&self) -> Result<(), BoxError>{
      let listener = TcpListener::bind("127.0.0.1:8080").await?;

      loop {
         let (conn, _) = listener.accept().await?;
         let codec = self.codec.clone();
         tokio::spawn(async move {
            init_channel(conn, codec).await;
         }).await?;
      }
   }
}

/// Create a `Channel` when a connection come on, and then create a handle chain.
async fn init_channel<Item: Debug, T: Debug + Clone + Encoder<Item> + Decoder>(socket: impl AsyncRead + AsyncWrite + Send + Unpin + 'static, codec: T) {
   println!("a new conn come in");
   let mut channel = Channel::new(socket, codec);
   while let Some(_v) = channel.next().await {
      // channel.send(v.unwrap());
      println!("world");
   }
}