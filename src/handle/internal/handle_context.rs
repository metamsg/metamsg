use slotmap::DefaultKey;

pub struct HandleContext<Handle> {
    handle: Handle,
    prev: Option<DefaultKey>,
    next: Option<DefaultKey>,
    config: HandleContextConfig,
    // arc or rc or weak?, 不可以循环引用，不同于netty，这里不持有channel，不持有channel如何操作channel
    // channel: Arc<Channel<Conn, Codec, Item>>,
}

pub struct HandleContextConfig {
    pub inbound: Option<bool>,
    pub outbound: Option<bool>,
}

impl<Handle> HandleContext<Handle> {
    pub fn new(handle: Handle) -> Self {
        HandleContext {
            handle,
            prev: None,
            next: None,
            // todo 从handle判断inbound和outbound
            config: HandleContextConfig::inbound(),
        }
    }

    pub fn set_prev(&mut self, prev: Option<DefaultKey>) {
        self.prev = prev
    }

    pub fn set_next(&mut self, next: Option<DefaultKey>) {
        self.next = next
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