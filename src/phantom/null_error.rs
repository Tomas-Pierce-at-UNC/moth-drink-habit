
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct NullError;

impl fmt::Display for NullError {

    fn fmt(&self, f :&mut fmt::Formatter<'_>) -> Result<(),fmt::Error> {
        write!(f, "Something was unexpectedly a null pointer")
    }

}

impl Error for NullError{}
