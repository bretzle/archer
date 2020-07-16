use winapi::um::winuser::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};

#[derive(Debug, Copy, Clone)]
pub struct Display {
	pub height: i32,
	pub width: i32,
}

impl Default for Display {
	fn default() -> Self {
		unsafe {
			Self {
				height: GetSystemMetrics(SM_CYSCREEN),
				width: GetSystemMetrics(SM_CXSCREEN),
			}
		}
	}
}
