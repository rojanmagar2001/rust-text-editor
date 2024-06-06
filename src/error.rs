//! # Error

#[derive(Debug)]
pub enum Error {
    /// Wrapper around `std::io::Error`
    Io(std::io::Error),
    /// Wrapper around `std::fmt::Error`
    Fmt(std::fmt::Error),
    /// Error returned when the window size obtained through a system call is invalid.
    InvalidWindowSize,
    /// Error setting or retreiving the cursor position.
    CursorPosition,
    /// Configuration error. The three attributes correspond the file path, line number and
    /// the error message.
    Config(std::path::PathBuf, usize, String),
    /// Too many arguments given to kibi. The attribute corresponds to the total number of command
    /// line arguments given.
    TooManyArguments(usize),
    /// Unrecognized command line option. The attribute corresponds to the option given.
    UnrecognizedOption(String),
}

impl From<std::io::Error> for Error {
    /// Convert an IO Error into this error type.
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<std::fmt::Error> for Error {
    /// Convert a formatting error into this error type.
    fn from(err: std::fmt::Error) -> Self {
        Self::Fmt(err)
    }
}
