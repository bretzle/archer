use std::{
	error::Error,
	fmt::{self, Display},
	marker::Sized,
};
use winapi::{
	shared::windef::RECT,
	um::winuser::{GetDesktopWindow, GetForegroundWindow, GetWindowRect},
};

pub type WinApiResult<T> = Result<T, WinApiError>;

#[derive(Debug)]
pub enum WinApiError {
	Err(i32),
	Null,
}

impl Error for WinApiError {}

impl Display for WinApiError {
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

pub fn is_fullscreen() -> bool {
	let mut a = RECT::default();
	let mut b = RECT::default();

	unsafe {
		let hwnd = GetForegroundWindow();
		GetWindowRect(hwnd, &mut a);
		GetWindowRect(GetDesktopWindow(), &mut b);
	}

	a.left == b.left && a.top == b.top && a.right == b.right && a.bottom == b.bottom
}
