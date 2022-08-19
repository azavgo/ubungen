
//Custom error
#[derive(Debug)]
pub enum UbungenError { 
    IOError(std::io::Error),  
}

impl From<std::io::Error> for UbungenError {
    fn from(error: std::io::Error) -> Self {
        UbungenError::IOError(error)
    }
}
