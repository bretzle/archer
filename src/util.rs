use thiserror::Error;

pub type WinApiResult<T> = Result<T, WinApiResultError>;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum WinApiResultError {
	#[error("Windows Api errored and returned a value of {0}")]
	Err(i32),
	#[error("Windows Api errored and returned a null value")]
	Null,
}

pub fn winapi_ptr_to_result<T>(input: *mut T) -> WinApiResult<*mut T> {
	if !input.is_null() {
		Ok(input)
	} else {
		Err(WinApiResultError::Null)
	}
}

pub fn winapi_nullable_to_result<T>(input: T) -> WinApiResult<T>
where
	T: PartialEq<i32>,
{
	if input != 0 {
		Ok(input)
	} else {
		Err(WinApiResultError::Null)
	}
}
