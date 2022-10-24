use std::fmt::{Debug, Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    BackendRequestError(String),
    TopicSubscriptionError(String),
    ClientConfigurationError(reqwest::Error),
    ClientError(String),
    VariableParseError,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.write_str(format!("{self:?}").as_ref())?;
        Ok(())
    }
}

impl std::error::Error for Error {}
