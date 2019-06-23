use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug)]
pub enum VTextError {
    SomeError,
}

impl VTextError {
    fn descr(&self) -> &str {
        match *self {
            VTextError::SomeError => "Some error message",
        }
    }
}

impl Error for VTextError {
    fn description(&self) -> &str {
        self.descr()
    }
}

impl fmt::Display for VTextError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.descr().fmt(f)
    }
}
