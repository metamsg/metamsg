use crate::handle::{Handle, InboundHandle};

pub struct LoggerHandler;

impl LoggerHandler {
    pub fn new() -> Self {
        LoggerHandler
    }
}

impl Handle for LoggerHandler {}

impl InboundHandle for LoggerHandler {
    type Input = String;
    type Output = String;

    fn read(input: Self::Input) -> Self::Output {
        println!("{:?}", input);
        input
    }

    fn write(output: Self::Output) -> Self::Input {
        println!("{:?}", output);
        output
    }
}
