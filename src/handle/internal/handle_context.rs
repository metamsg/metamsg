use std::any::Any;
use crate::handle::internal::handle_chain::ShareChain;
use crate::handle::{InboundHandle, OutboundHandle};
use futures::{Sink, Stream};
use pin_project_lite::pin_project;
use slotmap::DefaultKey;
use std::fmt::Debug;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio_util::codec::Encoder;

pin_project! {
    #[derive(Debug)]
    pub struct HandleContext<Handle, Conn, Codec, Item> {
        handle: Handle,
        prev: Option<DefaultKey>,
        next: Option<DefaultKey>,
        config: HandleContextConfig,
        #[pin]
        chain: ShareChain<Handle, Conn, Codec, Item>,
        // arc or rc or weak?, 不可以循环引用，不同于netty，这里不持有channel，不持有channel如何操作channel
        // channel: Arc<Channel<Conn, Codec, Item>>,
    }
}

#[derive(Debug)]
pub struct HandleContextConfig {
    pub inbound: Option<bool>,
    pub outbound: Option<bool>,
}

impl<Handle, Conn, Codec, Item> HandleContext<Handle, Conn, Codec, Item> {
    pub fn new(handle: Handle, chain: ShareChain<Handle, Conn, Codec, Item>) -> Self {
        HandleContext {
            handle,
            prev: None,
            next: None,
            // todo 从handle判断inbound和outbound
            config: HandleContextConfig::inbound(),
            chain,
        }
    }

    pub fn set_prev(&mut self, prev: Option<DefaultKey>) {
        self.prev = prev
    }

    pub fn set_next(&mut self, next: Option<DefaultKey>) {
        self.next = next
    }
}

impl<Handle, Conn, Codec, Item> Stream for HandleContext<Handle, Conn, Codec, Item>
where
    Handle: InboundHandle
{
    type Item = Box<dyn Any>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.as_mut()
            .project()
            .chain
            .get_next(self.next)
            .poll_next(cx)
            .map(|opt| opt.map(|x| (Handle::read(x))))
    }
}

impl<Handle, Conn, Codec, Item> Sink<Item> for HandleContext<Handle, Conn, Codec, Item>
where
    Handle: OutboundHandle,
    Codec: Encoder<Item>,
    <Codec as Encoder<Item>>::Error: From<io::Error> + Debug,
    Item: Debug + Send + 'static,
{
    type Error = Codec::Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().chain.get_next(self.next).poll_ready(cx)
    }

    fn start_send(self: Pin<&mut Self>, item: Item) -> Result<(), Self::Error> {
        self.project().chain.get_next(self.next).start_send(item)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().chain.get_next(self.next).poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().chain.get_next(self.next).poll_close(cx)
    }
}

impl HandleContextConfig {
    pub fn inbound() -> Self {
        HandleContextConfig {
            inbound: Some(true),
            outbound: Some(false),
        }
    }
}
