use std::fmt::{Formatter, Display};

// represents potential erros that can occur
pub enum ConnectionError {
    // indicates an error ocurred while creating WS server
    CreateServerError(String), // stores a description of the creation server error
}

impl Display for ConnectionError {

    fn fmt(&self, f: &mut Formatter <'_>) -> std::fmt::Result {
        match *self{
            ConnectionError::CreateServerError(ref desc) => {
                write!(f, "Error while creating server. {}", desc)
            }
        }
    }
}