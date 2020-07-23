use std::{error::Error, fmt};

pub type WinApiResult<T> = Result<T, WinApiError>;

#[derive(Debug)]
pub enum WinApiError {
	Err(i32),
	Null,
}

impl Error for WinApiError {}

impl fmt::Display for WinApiError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match *self {
			WinApiError::Err(num) => {
				write!(f, "Windows Api errored and returned a value of {}", num)
			}
			WinApiError::Null => write!(f, "Windows Api errored and returned a null value"),
		}
	}
}

pub trait CTypeExt {
	fn as_result(self) -> WinApiResult<Self>
	where
		Self: Sized;
}

pub trait PtrExt {
	fn as_result(self) -> WinApiResult<Self>
	where
		Self: Sized;
}

impl<T> CTypeExt for T
where
	T: Into<i32>,
	T: Copy,
{
	fn as_result(self) -> WinApiResult<Self> {
		if self.into() != 0 {
			Ok(self)
		} else {
			Err(WinApiError::Null)
		}
	}
}

impl<T> PtrExt for *mut T {
	fn as_result(self) -> WinApiResult<Self>
	where
		Self: Sized,
	{
		if !self.is_null() {
			Ok(self)
		} else {
			Err(WinApiError::Null)
		}
	}
}
