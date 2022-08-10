use crate::handle::{Handle, InboundHandle, OutboundHandle};
use crate::Channel;
use futures::{Sink, Stream};
use pin_project_lite::pin_project;
use slotmap::DefaultKey;
use std::fmt::Debug;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_util::codec::{Decoder, Encoder};

pin_project! {
    #[derive(Debug)]
    pub(crate) struct HeadContext<Conn, Codec, Item> {
        #[pin]
        channel: Channel<Conn, Codec, Item>,
        next: Option<DefaultKey>,
    }
}

impl<Conn, Codec, Item> HeadContext<Conn, Codec, Item> {
    pub fn new(channel: Channel<Conn, Codec, Item>) -> Self {
        Self {
            channel,
            next: None,
        }
    }

    pub fn set_next(&mut self, next: Option<DefaultKey>) {
        self.next = next;
    }
}

impl<Conn, Codec, Item> Handle for HeadContext<Conn, Codec, Item> {}

impl<Conn, Codec, Item> InboundHandle for HeadContext<Conn, Codec, Item>
where
    Conn: AsyncRead,
    Codec: Decoder,
    <Codec as Decoder>::Error: From<io::Error> + Debug,
    <Codec as Decoder>::Item: Debug,
{
    type Input = Result<<Codec as Decoder>::Item, <Codec as Decoder>::Error>;

    type Output = Result<<Codec as Decoder>::Item, <Codec as Decoder>::Error>;

    fn read(input: Self::Input) -> Self::Output {
        todo!()
    }

    fn write(output: Self::Output) -> Self::Input {
        todo!()
    }
}

impl<Conn, Codec, Item> OutboundHandle for HeadContext<Conn, Codec, Item>
where
    Conn: AsyncWrite,
    Codec: Encoder<Item>,
    <Codec as Encoder<Item>>::Error: From<io::Error> + Debug,
    Item: Debug + Send + 'static,
{
    type Input = Item;

    type Output = Item;

    fn read(input: Self::Input) -> Self::Output {
        todo!()
    }

    fn write(output: Self::Output) -> Self::Input {
        todo!()
    }
}

impl<Conn, Codec, Item> Stream for HeadContext<Conn, Codec, Item>
where
    Conn: AsyncRead,
    Codec: Decoder,
    <Codec as Decoder>::Error: From<io::Error> + Debug,
    <Codec as Decoder>::Item: Debug,
{
    type Item = <Self as InboundHandle>::Output;

    // 在这里对接inbound handle，将消息发送到handle链里
    // fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    //     self.project().channel.poll_next(cx)
    // }

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.as_mut()
            .project()
            .channel
            .poll_next(cx)
            .map(|opt| opt.map(|x| (<Self as InboundHandle>::read(x))))
    }
}

impl<Conn, Codec, Item> Sink<Item> for HeadContext<Conn, Codec, Item>
where
    Conn: AsyncWrite,
    Codec: Encoder<Item>,
    <Codec as Encoder<Item>>::Error: From<io::Error> + Debug,
    Item: Debug + Send + 'static,
{
    type Error = Codec::Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().channel.poll_ready(cx)
    }

    fn start_send(self: Pin<&mut Self>, item: Item) -> Result<(), Self::Error> {
        self.project().channel.start_send(item)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().channel.poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().channel.poll_close(cx)
    }
}
