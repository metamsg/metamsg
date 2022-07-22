use std::fmt::Debug;
use std::io;
use pin_project_lite::pin_project;
use tokio_util::codec::{Decoder, Encoder, Framed};
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};
use futures::{Sink, Stream};
use tokio::io::{AsyncRead, AsyncWrite};

pin_project! {
    /// Every `Channel` hold a `Framed` and a `Stream`
    #[derive(Debug)]
    pub struct Channel<Conn, Codec, Item> {
        #[pin]
        frame: Framed<Conn, Codec>,
        _phantom: PhantomData<Item>,
    }
}

pub enum ChannelStatus {
    Open,
    Register,
    Active,
}

impl <Conn, Codec, Item> Channel<Conn, Codec, Item>
where
    Conn: AsyncRead + AsyncWrite + Send + Unpin + 'static,
    Codec: Debug + Clone + Encoder<Item> + Decoder,
    <Codec as Encoder<Item>>::Error: From<io::Error> + Debug,
    <Codec as Decoder>::Error: From<io::Error> + Debug,
{
    /// Create a channel with Socket and codec.
    pub fn new(conn: Conn, codec: Codec) -> Self {
        Channel {
            frame: Framed::new(conn, codec),
            _phantom: PhantomData::default(),
        }
    }

    /// shutdown
    pub fn shutdown() {}

    /// is_active
    pub fn is_active() {}
}

impl<Conn, Codec, Item> Stream for Channel<Conn, Codec, Item>
where
    Conn: AsyncRead,
    Codec: Decoder,
    Codec::Error: From<io::Error> + Debug,
{
    type Item = Result<Codec::Item, Codec::Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.project().frame.poll_next(cx)
    }
}

impl<Conn, Codec, Item> Sink<Item> for Channel<Conn, Codec, Item>
where
    Conn: AsyncWrite,
    Codec: Encoder<Item>,
    Codec::Error: From<io::Error> + Debug,
{
    type Error = Codec::Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().frame.poll_ready(cx)
    }

    fn start_send(self: Pin<&mut Self>, item: Item) -> Result<(), Self::Error> {
        self.project().frame.start_send(item)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().frame.poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().frame.poll_close(cx)
    }
}
