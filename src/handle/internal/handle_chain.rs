use crate::handle::internal::handle_context::HandleContext;
use crate::handle::internal::head_context::HeadContext;
use crate::handle::internal::tail_context::TailContext;
use crate::Channel;
use slotmap::{DefaultKey, SlotMap};
use std::borrow::BorrowMut;
use std::pin::Pin;
use std::sync::Arc;

/// A channel create, then initial a chain with handle contexts
// Consider if need contexts, because of there are streams.
// Need, more flexible.
#[derive(Debug)]
pub struct Chain<Handle, Conn, Codec, Item> {
    handles: SlotMap<DefaultKey, HandleContext<Handle, Conn, Codec, Item>>,
    head: HeadContext<Conn, Codec, Item>,
    tail: TailContext,
}

#[derive(Debug)]
pub struct ShareChain<Handle, Conn, Codec, Item>(Arc<Chain<Handle, Conn, Codec, Item>>);

impl<Handle, Conn, Codec, Item> Clone for ShareChain<Handle, Conn, Codec, Item> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<Handle, Conn, Codec, Item> ShareChain<Handle, Conn, Codec, Item> {
    pub fn new(channel: Channel<Conn, Codec, Item>) -> Self {
        Self(Arc::new(Chain {
            handles: SlotMap::new(),
            head: HeadContext::new(channel),
            tail: TailContext::new(),
        }))
    }

    pub fn add_last(&mut self, handle: Handle) {
        let mut handle_context = HandleContext::new(handle, self.clone());
        handle_context.set_prev(None);
        handle_context.set_next(None);
        let key = self.0.handles.insert(handle_context);
        self.0.head.set_next(Some(key));
        self.0.tail.set_prev(Some(key));
    }

    pub fn get_next(
        &self,
        key: Option<DefaultKey>,
    ) -> Pin<&mut HandleContext<Handle, Conn, Codec, Item>> {
        Pin::new(self.0.handles.get(key.unwrap()).unwrap().borrow_mut())
    }
}
