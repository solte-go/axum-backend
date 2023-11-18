use serde::Serialize;

use crate::utils;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
    // -- Key
    KeyFailHmac,
    // -- PWD
    PwdNotMatching,
    // -- Token
    TokenInvalidFormat,
    TokenCannotDecodeIdent,
    TokenCannotDecodeExp,
    TokenSignatureNotMatching,
    TokenExpNotIso,
    TokenExpired,
    //Utils
    Utils(utils::Error),

}

impl core::fmt::Display for Error {
    fn fmt(
        &self, 
        fmt: &mut core::fmt::Formatter 
    ) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl From<utils::Error> for Error {
	fn from(value: utils::Error) -> Self {
		Self::Utils(value)
	}
}