
#[derive(Debug, Clone)]
pub enum Error {
    Prepare(String),
    Input(String),
    Output(String),
    Request(String),
    ParseFunction(String),
    ExecuteFunction(String),
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::ExecuteFunction(e.to_string())
    }
}