use slotmap::DefaultKey;

#[derive(Debug)]
pub(crate) struct TailContext {
    prev: Option<DefaultKey>,
}

impl TailContext {
    pub fn new() -> Self {
        TailContext { prev: None }
    }

    pub fn set_prev(&mut self, prev: Option<DefaultKey>) {
        self.prev = prev
    }
}
