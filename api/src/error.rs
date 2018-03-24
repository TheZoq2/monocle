pub type Result<T> = ::core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// The length of the buffer was expected to be $1 but was only $2
    BufferTooSmall(usize, usize),
}
