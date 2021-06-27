#[derive(Debug)]
pub struct FlushError(pub String, pub usize, pub String);

pub type Result<T> = std::result::Result<T, FlushError>;
