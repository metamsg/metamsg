use crate::handle::internal::handle_context::HandleContext;
use crate::handle::internal::head_context::HeadContext;
use crate::handle::internal::tail_context::TailContext;
use slotmap::{DefaultKey, SlotMap};

/// A channel create, then initial a chain with handle contexts
// Consider if need contexts, because of there are streams.
// Need, more flexible.
pub struct Chain<Handle> {
    handles: SlotMap<DefaultKey, HandleContext<Handle>>,
    head: HeadContext,
    tail: TailContext,
}

impl<Handle> Chain<Handle> {
    pub fn new() -> Self {
        Self {
            handles: SlotMap::new(),
            head: HeadContext::new(),
            tail: TailContext::new(),
        }
    }

    pub fn add_last(&mut self, handle: Handle) {
        let mut handle_context = HandleContext::new(handle);
        handle_context.set_prev(None);
        handle_context.set_next(None);
        let key = self.handles.insert(handle_context);
        self.head.set_next(Some(key));
        self.tail.set_prev(Some(key));
    }
}
