use crate::handle::{Handle, InboundHandle};

pub struct LoggerHandler;

impl LoggerHandler {
    pub fn new() -> Self {
        LoggerHandler
    }
}

impl Handle for LoggerHandler {}

impl InboundHandle for LoggerHandler {

    fn read(input: String) -> String {
        println!("{:?}", input);
        input
    }
}
