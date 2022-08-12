pub(crate) mod internal;
pub mod logger_handler;
pub mod make_handle;

/// Some process like channel control, channel event notice and error.
/// Inbound and outbound handle are different.
/// Consider use function or method, stream usually use function.
pub trait Handle {}

pub trait InboundHandle<T>: Handle {
    type Output;

    fn read(input: T) -> Self::Output;
}

pub trait OutboundHandle<T>: Handle {
    type Output;

    fn write(input: T) -> Self::Output;
}
