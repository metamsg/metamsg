use std::sync::Arc;
use slotmap::{DefaultKey, SlotMap};
use crate::Channel;

/// a channel create, then initial a chain with handle contexts
// consider if need contexts, because of there are streams.
pub struct Chain<Handle, Conn, Codec, Item> {
    handle: SlotMap<DefaultKey, HandleContext<Handle, Conn, Codec, Item>>,
    head: Option<DefaultKey>,
    tail: Option<DefaultKey>,
}

pub struct HandleContext<Handle, Conn, Codec, Item> {
    value: Handle,
    prev: Option<DefaultKey>,
    next: Option<DefaultKey>,
    // arc or rc or weak?, 不可以循环引用，不同于netty，这里不持有channel，不持有channel如何操作channel
    channel: Arc<Channel<Conn, Codec, Item>>,
    pub inbound: bool,
    pub outbound: bool,
}