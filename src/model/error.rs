use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

use super::{store, crypt};

pub type Result<T> = core::result::Result<T, Error>;

// TODO learn about "serde_as" "DisplayFromStr"
#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
	EntryNotFound { entry: &'static str, id: i64},
	TransactionError(String),

	// -- Modules
	Store(store::Error),

	// -- Crypt
	Crypt(crypt::Error),

	// -- Externals
	Sqlx(#[serde_as(as = "DisplayFromStr")]sqlx::Error)
}

impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}


impl From<store::Error> for Error {
	fn from(value: store::Error) -> Self {
		Self::Store(value)
	}
}

impl From<crypt::Error> for Error {
	fn from(value: crypt::Error) -> Self {
		Self::Crypt(value)
	}
}

impl From<sqlx::Error> for Error {
	fn from(value: sqlx::Error) -> Self {
		Self::Sqlx(value)
	}
}