use std::{error::Error, fmt::Display};

pub type WinResult<T> = Result<T, WinError>;

#[derive(Debug)]
pub enum WinError {
	Err(i32),
	Null,
}

impl Display for WinError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			WinError::Err(id) => write!(f, "Windows Api errored and returned a value of {}", id),
			WinError::Null => write!(f, "Windows Api errored and returned a null value"),
		}
	}
}

impl Error for WinError {}
