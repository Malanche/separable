use syn::{Error as SynError};
use proc_macro2::Span;

#[derive(Debug)]
pub enum ErrorKind {
    Custom(String)
}

impl ErrorKind {
    pub fn custom<A: Into<String>>(message: A) -> Self {
        ErrorKind::Custom(message.into())
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let content = match self {
            ErrorKind::Custom(e) => format!("{}", e)
        };
        write!(formatter, "{}", content)
    }
}

#[derive(Debug)]
pub struct Error {
    pub span: Span,
    pub kind: ErrorKind
}

impl From<Error> for SynError {
    fn from(error: Error) -> SynError {
        SynError::new(error.span, format!("{}", error.kind))
    }
}

impl Error {
    pub fn custom<A: Into<String>>(message: A, span: Span) -> Self {
        Error {
            kind: ErrorKind::Custom(message.into()),
            span
        }
    }

    pub fn to_syn_error(self) -> SynError {
        self.into()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(formatter, "{}", self.kind)
    }
}

impl std::error::Error for Error {}