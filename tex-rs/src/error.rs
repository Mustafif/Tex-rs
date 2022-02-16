use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

/// TexError that is used for custom error handling 
#[derive(Debug)]
pub enum TexError{
    /// Error in priority ranking 
    RankError,
}

impl Display for TexError{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            TexError::RankError => write!(f, "Rank Error"),
        }
    }
}

impl Error for TexError{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self{
            TexError::RankError => None,
        }
    }
}