pub(crate) mod internal;
pub mod logger_handler;
pub mod make_handle;

/// Some process like channel control, channel event notice and error.
/// Inbound and outbound handle are different.
/// Consider use function or method, stream usually use function.
pub trait Handle {
    type Input;

    type Output;
}

pub trait InboundHandle: Handle {
    fn read(input: Self::Input) -> Self::Output;

    fn write(output: Self::Output) -> Self::Input;
}

pub trait OutboundHandle: Handle {
    fn read(input: Self::Input) -> Self::Output;

    fn write(output: Self::Output) -> Self::Input;
}
