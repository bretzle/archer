use winapi::um::winuser::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};

#[derive(Default)]
pub struct Display {
	pub height: i32,
	pub width: i32,
}

impl Display {
	pub fn new() -> Self {
		unsafe {
			Self {
				height: GetSystemMetrics(SM_CYSCREEN),
				width: GetSystemMetrics(SM_CXSCREEN),
			}
		}
	}
}
