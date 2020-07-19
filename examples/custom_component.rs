use appbar::prelude::*;
use std::{ffi::CString, thread, time::Duration};
use winapi::{
	shared::windef::{HWND, RECT, SIZE},
	um::{
		wingdi::{GetTextExtentPoint32A, SetBkColor, SetTextColor},
		winuser::{DrawTextA, GetClientRect, GetDC, DT_CENTER, DT_SINGLELINE, DT_VCENTER},
	},
};

fn main() {
	simple_logger::init().unwrap();

	let bar = AppBar::create().with_component(Box::new(Custom::default()));

	bar.start();

	loop {
		thread::sleep(Duration::from_millis(1000));
	}
}

#[derive(Debug, Default)]
struct Custom {}

impl Component for Custom {
	fn setup(&self) {}

	fn interval(&self) -> Duration {
		Duration::from_millis(950)
	}

	fn draw(&self, hwnd: HWND) -> Result<(), WinApiError> {
		let mut rect = RECT::default();

		unsafe {
			GetClientRect(hwnd, &mut rect).as_result()?;
			let text = format!("{}", chrono::Utc::now().format("%T"));
			let text_len = text.len() as i32;
			let c_text = CString::new(text).unwrap();

			// Getting the device context
			let hdc = GetDC(hwnd).as_result()?;

			set_font(hdc);

			let mut size = SIZE::default();

			GetTextExtentPoint32A(hdc, c_text.as_ptr(), text_len, &mut size).as_result()?;

			rect.left = 1920 / 2 - (size.cx / 2) - 10;
			rect.right = 1920 / 2 + (size.cx / 2) + 10;

			//TODO: handle error
			SetTextColor(hdc, 0x00ffffff);
			SetBkColor(hdc, 0x2C2427);

			// Writing the time
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
		"Custom".to_owned()
	}
}
