use std::marker::Sized;
use thiserror::Error;

pub type WinApiResult<T> = Result<T, WinApiError>;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum WinApiError {
	#[error("Windows Api errored and returned a value of {0}")]
	Err(i32),
	#[error("Windows Api errored and returned a null value")]
	Null,
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
	T: PartialEq<i32>,
{
	fn as_result(self) -> WinApiResult<Self> {
		if self != 0 {
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
