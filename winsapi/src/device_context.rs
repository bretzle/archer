use crate::{error_handler::*, Font, TextOptions};
use std::ffi::CString;
use winapi::{
	shared::windef::{HDC, HWND, RECT, SIZE},
	um::{
		wingdi::{GetTextExtentPoint32A, SelectObject, SetBkColor, SetTextColor},
		winuser::{DrawTextA, GetClientRect, GetDC},
	},
};

pub struct DeviceContext {
	pub rect: RECT,
	hdc: HDC,
}

impl DeviceContext {
	pub fn new(hwnd: HWND) -> WinApiResult<Self> {
		unsafe {
			let mut rect = RECT::default();
			GetClientRect(hwnd, &mut rect).as_result()?;
			let hdc = GetDC(hwnd).as_result()?;

			Ok(Self { rect, hdc })
		}
	}

	pub fn set_font(&self, font: Font) {
		unsafe {
			SelectObject(self.hdc, font.0 as *mut winapi::ctypes::c_void);
		}
	}

	pub fn set_text_color(&self, color: u32) {
		unsafe {
			SetTextColor(self.hdc, color);
		}
	}

	pub fn set_background_color(&self, color: u32) {
		unsafe {
			SetBkColor(self.hdc, color);
		}
	}

	pub fn get_text_extent(&self, text: String) -> WinApiResult<SIZE> {
		let mut size = SIZE::default();
		unsafe {
			GetTextExtentPoint32A(
				self.hdc,
				CString::new(text.clone()).unwrap().as_ptr(),
				text.len() as i32,
				&mut size,
			)
			.as_result()?;
		}
		Ok(size)
	}

	pub fn draw_text(&mut self, text: String, options: TextOptions) -> WinApiResult<()> {
		let len = text.len() as i32;
		let cstring = CString::new(text).unwrap();
		unsafe {
			DrawTextA(self.hdc, cstring.as_ptr(), len, &mut self.rect, options.0)
				.as_result()
				.map(|_| ())
		}
	}
}
