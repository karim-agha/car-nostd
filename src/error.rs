use alloc::string::{String, ToString};

pub enum Error {
    Parsing(String),
    InvalidFile(String),
    Io(core2::io::Error),
    Cbor(serde_ipld_dagcbor::error::CodecError),
    LdReadTooLarge(usize),
}

impl core::fmt::Debug for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Parsing(s) => write!(f, "Parsing error: {}", s),
            Error::InvalidFile(s) => write!(f, "Invalid file error: {}", s),
            Error::Io(e) => write!(f, "Io error: {}", e),
            Error::Cbor(e) => write!(f, "Cbor error: {}", e),
            Error::LdReadTooLarge(s) => write!(f, "Ld read too large: {}", s),
        }
    }
}

impl From<core2::io::Error> for Error {
    fn from(err: core2::io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<cid::Error> for Error {
    fn from(err: cid::Error) -> Error {
        Error::Parsing(err.to_string())
    }
}

impl From<cid::multihash::Error> for Error {
    fn from(err: cid::multihash::Error) -> Error {
        Error::Parsing(err.to_string())
    }
}
