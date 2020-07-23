use crate::Rect;
use std::{mem, ptr};
use winapi::{
	shared::windef::HWND,
	um::winuser::{
		GetForegroundWindow, GetWindowInfo, SetWindowPos, ShowWindow, SWP_NOACTIVATE, SW_MINIMIZE,
		SW_RESTORE, WINDOWINFO,
	},
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

	pub fn set_pos(&mut self, rect: Rect, insert_after: Option<Window>) {
		unsafe {
			SetWindowPos(
				self.0,
				insert_after.unwrap_or_default().0,
				rect.x,
				rect.y,
				rect.w,
				rect.h,
				SWP_NOACTIVATE,
			);
		}
	}

	/// Get's info about the window
	pub unsafe fn info(self) -> WindowInfo {
		let mut info: WINDOWINFO = mem::zeroed();
		info.cbSize = mem::size_of::<WINDOWINFO>() as u32;

		GetWindowInfo(self.0, &mut info);

		info.into()
	}

	/// Get's the dimensions of the window without the border
	pub fn transparent_border(self) -> (i32, i32) {
		let info = unsafe { self.info() };

		let x = {
			(info.window_rect.x - info.client_rect.x) + (info.window_rect.w - info.client_rect.w)
		};

		let y = {
			(info.window_rect.y - info.client_rect.y) + (info.window_rect.h - info.client_rect.h)
		};

		(x, y)
	}

	/// Restores the window to it's previous location
	pub fn restore(&mut self) {
		unsafe {
			ShowWindow(self.0, SW_RESTORE);
		};
	}

	/// Minimizes the window
	pub fn minimize(&mut self) {
		unsafe {
			ShowWindow(self.0, SW_MINIMIZE);
		}
	}
}

impl Default for Window {
	fn default() -> Self {
		Self(ptr::null_mut())
	}
}

impl PartialEq for Window {
	fn eq(&self, other: &Window) -> bool {
		self.0 == other.0
	}
}

/// Info about the window
#[derive(Debug)]
pub struct WindowInfo {
	/// Dimension's of the window
	pub window_rect: Rect,
	/// Dimension's of the actual window
	pub client_rect: Rect,
	/// styles
	pub styles: u32,
	/// extended styles
	pub extended_styles: u32,
	/// x borders
	pub x_borders: u32,
	/// y borders
	pub y_borders: u32,
}

impl From<WINDOWINFO> for WindowInfo {
	fn from(info: WINDOWINFO) -> Self {
		WindowInfo {
			window_rect: info.rcWindow.into(),
			client_rect: info.rcClient.into(),
			styles: info.dwStyle,
			extended_styles: info.dwExStyle,
			x_borders: info.cxWindowBorders,
			y_borders: info.cxWindowBorders,
		}
	}
}
