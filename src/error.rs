// import needed libraries
use std::fmt;

// define common lib's errors for end user
pub enum Error {
    TooManyGradientPointsRequested
}

// make sure the user can use .unwrap command
impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Error::TooManyGradientPointsRequested => "You asked for a noise with too many gradient points compared to the width of the picture 🤕"
        })
    }
}
