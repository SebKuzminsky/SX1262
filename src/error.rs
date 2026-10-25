#[derive(Debug, thiserror::Error)]
pub enum SerializationError {
    #[error(transparent)]
    Infallible(#[from] core::convert::Infallible),
}

#[derive(Debug, thiserror::Error)]
pub enum DeserializationError {
    #[error(transparent)]
    Infallible(#[from] core::convert::Infallible),
    #[error(transparent)]
    Status(#[from] crate::StatusError),
}

pub type CommandError<SPI> = regiface::errors::CommandError<
    <SPI as embedded_hal_async::spi::ErrorType>::Error,
    SerializationError,
    DeserializationError,
>;
