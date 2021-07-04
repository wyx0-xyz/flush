use ansi_term::Colour::{Blue, Red, Yellow};
use std::fmt;

#[derive(Debug)]
pub struct FlushError(pub String, pub usize, pub String, pub Option<String>);

pub type Result<T> = std::result::Result<T, FlushError>;

impl fmt::Display for FlushError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "=> {}:{}\n{}: {}",
            Blue.paint(self.0.clone()),
            Yellow.paint(self.1.to_string()),
            Red.paint("error"),
            self.2
        )?;

        match self.3.clone() {
            Some(note) => write!(f, "\n{}: {}", Yellow.paint("note"), note),
            None => write!(f, ""),
        }
    }
}
