use crate::{app_bar::RedrawAppBarReason, event::Event, util::WinApiError, AppBar, Component};
use std::thread;
use winapi::shared::windef::HWND;

#[derive(Debug, Default)]
pub struct Clock {}

impl Component for Clock {
	fn setup(&self) {
		thread::spawn(|| loop {
			thread::sleep(std::time::Duration::from_millis(950));
			if AppBar::get().window == 0 {
				break;
			}
			AppBar::send_message(Event::RedrawAppBar(RedrawAppBarReason::Time))
				.expect("Failed to send redraw-app-bar event");
		});
	}

	fn draw(&self, _hwnd: HWND) -> Result<(), WinApiError> {
		println!("Drawing clock");

		Ok(())
	}
}
