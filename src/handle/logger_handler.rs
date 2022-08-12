use crate::handle::{Handle, InboundHandle};

pub struct LoggerHandler;

impl LoggerHandler {
    pub fn new() -> Self {
        LoggerHandler
    }
}

impl Handle for LoggerHandler {}

impl InboundHandle<String> for LoggerHandler {
    type Output = String;

    fn read(input: String) -> Self::Output {
        println!("{:?}", input);
        input
    }
}
