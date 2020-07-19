use appbar::prelude::*;
use std::{thread, time::Duration};
use winapi::um::winuser::{DT_CENTER, DT_SINGLELINE, DT_VCENTER};
use winsapi::{DeviceContext, WinApiError};

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
	fn interval(&self) -> Duration {
		Duration::from_millis(950)
	}

	fn draw(&self, data: &DrawData, mut dc: DeviceContext) -> Result<(), WinApiError> {
		let text = format!("{}", chrono::Utc::now().format("%T"));

		dc.set_font(*data.font);
		let size = dc.get_text_extent(text.clone())?;

		dc.rect.left = data.display.width / 2 - (size.cx / 2) - 10;
		dc.rect.right = data.display.width / 2 + (size.cx / 2) + 10;

		dc.set_text_color(0x00ffffff);
		dc.set_background_color(*data.bg_color as u32);

		dc.draw_text(text, DT_CENTER | DT_VCENTER | DT_SINGLELINE)?;

		Ok(())
	}

	fn reason(&self) -> RedrawReason {
		"Custom".to_owned()
	}
}
