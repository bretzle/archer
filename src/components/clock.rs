use crate::{
	app_bar::{set_font, RedrawReason},
	event::{Event, EventSender},
	util::*,
	Component, INSTANCE,
};
use std::{ffi::CString, thread};
use winapi::{
	shared::windef::{HWND, RECT, SIZE},
	um::{
		wingdi::{GetTextExtentPoint32A, SetBkColor, SetTextColor},
		winuser::{DrawTextA, GetClientRect, GetDC, DT_CENTER, DT_SINGLELINE, DT_VCENTER},
	},
};

#[derive(Debug, Default)]
pub struct Clock {}

impl Component for Clock {
	fn setup(&self, window: &'static i32, channel: EventSender) {
		thread::spawn(move || loop {
			thread::sleep(std::time::Duration::from_millis(950));
			if *window == 0 {
				break;
			}
			channel
				.send(Event::RedrawAppBar("Time".to_owned()))
				.expect("Failed to send redraw-app-bar event");
		});
	}

	fn draw(&self, hwnd: HWND) -> Result<(), WinApiError> {
		let mut rect = RECT::default();

		unsafe {
			GetClientRect(hwnd, &mut rect).as_result()?;
			let text = format!("{}", chrono::Local::now().format("%T"));
			let text_len = text.len() as i32;
			let c_text = CString::new(text).unwrap();
			let display = INSTANCE.get().unwrap().display;
			let config = INSTANCE.get().unwrap().config;

			// Getting the device context
			let hdc = GetDC(hwnd).as_result()?;

			set_font(hdc);

			let mut size = SIZE::default();

			GetTextExtentPoint32A(hdc, c_text.as_ptr(), text_len, &mut size).as_result()?;

			rect.left = display.width / 2 - (size.cx / 2) - 10;
			rect.right = display.width / 2 + (size.cx / 2) + 10;

			//TODO: handle error
			SetTextColor(hdc, 0x00ffffff);
			SetBkColor(hdc, config.bg_color as u32);

			// Writing the time
			DrawTextA(
				hdc,
				c_text.as_ptr(),
				text_len,
				&mut rect,
				DT_CENTER | DT_VCENTER | DT_SINGLELINE,
			)
			.as_result()?;

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
		"Time".to_owned()
	}
}
