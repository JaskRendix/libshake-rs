#[derive(Debug, Clone, Copy)]
pub enum ShakeError {
    Support,
    Device,
    Effect,
    Query,
    Arg,
    Transfer,
    Io,
    Permission,
}

pub type ShakeResult<T> = Result<T, ShakeError>;
