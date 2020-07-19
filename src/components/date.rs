use crate::{
	app_bar::{set_font, RedrawReason},
	util::*,
	Component, INSTANCE,
};
use std::{ffi::CString, time::Duration};
use winapi::{
	shared::windef::{HWND, RECT, SIZE},
	um::{
		wingdi::{GetTextExtentPoint32A, SetBkColor, SetTextColor},
		winuser::{DrawTextA, GetClientRect, GetDC, DT_CENTER, DT_SINGLELINE, DT_VCENTER},
	},
};

#[derive(Debug, Default)]
pub struct Date {}

impl Component for Date {
	fn setup(&self) {}

	fn interval(&self) -> Duration {
		Duration::from_millis(5000)
	}

	fn draw(&self, hwnd: HWND) -> Result<(), WinApiError> {
		let mut rect = RECT::default();

		unsafe {
			GetClientRect(hwnd, &mut rect).as_result()?;
			let display = INSTANCE.get().unwrap().display;
			let config = INSTANCE.get().unwrap().config;

			// Getting the device context
			let hdc = GetDC(hwnd).as_result()?;

			set_font(hdc);

			let mut size = SIZE::default();

			//TODO: handle error
			SetTextColor(hdc, 0x00ffffff);
			SetBkColor(hdc, config.bg_color as u32);

			let text = format!("{}", chrono::Local::now().format("%e %b %Y"));
			let text_len = text.len() as i32;
			let c_text = CString::new(text).unwrap();

			GetTextExtentPoint32A(hdc, c_text.as_ptr(), text_len, &mut size).as_result()?;

			rect.right = display.width - 10;
			rect.left = rect.right - size.cx;

			// Writing the date
			DrawTextA(
				hdc,
				c_text.as_ptr(),
				text_len,
				&mut rect,
				DT_CENTER | DT_VCENTER | DT_SINGLELINE,
			)
			.as_result()?;
		}

		Ok(())
	}

	fn reason(&self) -> RedrawReason {
		"Date".to_owned()
	}
}
