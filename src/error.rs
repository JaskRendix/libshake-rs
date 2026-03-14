#[derive(Debug, Clone, Copy)]
pub enum ShakeError {
    Support,
    Device,
    Effect,
    Query,
    Arg,
    Transfer,
}

pub type ShakeResult<T> = Result<T, ShakeError>;
