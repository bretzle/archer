use crate::Rect;
use std::ptr;
use winapi::{
	shared::windef::HWND,
	um::winuser::{GetForegroundWindow, SetWindowPos, SWP_NOACTIVATE},
};

#[derive(Debug, Copy, Clone)]
pub struct Window(pub HWND);

unsafe impl Send for Window {}
unsafe impl Sync for Window {}

impl Window {
	pub fn get_foreground_window() -> Self {
		let handle = unsafe { GetForegroundWindow() };
		Self(handle)
	}

	pub fn set_pos(&mut self, rect: Rect) {
		unsafe {
			SetWindowPos(
				self.0,
				0 as HWND,
				rect.x,
				rect.y,
				rect.w,
				rect.h,
				SWP_NOACTIVATE,
			);
		}
	}
}

impl Default for Window {
	fn default() -> Self {
		Self(ptr::null_mut())
	}
}
