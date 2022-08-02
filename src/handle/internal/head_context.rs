use slotmap::{DefaultKey};

pub(crate) struct HeadContext {
    next: Option<DefaultKey>
}

impl HeadContext {
    pub fn new() -> Self {
        Self {
            next: None,
        }
    }

    pub fn set_next(&mut self, next: Option<DefaultKey>) {
        self.next = next
    }
}