use std::any::Any;

pub(crate) mod internal;
pub mod logger_handler;
pub mod make_handle;

/// Some process like channel control, channel event notice and error.
/// Inbound and outbound handle are different.
/// Consider use function or method, stream usually use function.
pub trait Handle {}

pub trait InboundHandle: Handle {

    fn read(input: Box<dyn Any>) -> Box<dyn Any>;
}

pub trait OutboundHandle: Handle {

    fn write(input: Box<dyn Any>) -> Box<dyn Any>;
}
