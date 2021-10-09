use ansi_term::Colour::{Blue, Red, Yellow};
use std::{fmt, path::PathBuf};

#[derive(Debug)]
pub struct FlushError(pub PathBuf, pub usize, pub String);

pub type Result<T> = std::result::Result<T, FlushError>;

impl fmt::Display for FlushError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "=> {}:{}\n{}: {}",
            Blue.paint(self.0.to_str().unwrap()),
            Yellow.paint(self.1.to_string()),
            Red.paint("error"),
            self.2
        )
    }
}
