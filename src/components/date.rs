use crate::{app_bar::RedrawReason, Component, DrawData};
use std::time::Duration;
use winapi::um::winuser::{DT_CENTER, DT_SINGLELINE, DT_VCENTER};
use winsapi::*;

#[derive(Debug, Default)]
pub struct Date {}

impl Component for Date {
	fn interval(&self) -> Duration {
		Duration::from_millis(5000)
	}

	fn draw(&self, data: &DrawData, mut dc: DeviceContext) -> WinApiResult<()> {
		let text = format!("{}", chrono::Local::now().format("%e %b %Y"));

		dc.set_font(*data.font);
		dc.set_text_color(0x00ffffff);
		dc.set_background_color(*data.bg_color as u32);

		let size = dc.get_text_extent(text.clone())?;

		dc.rect.right -= 10;
		dc.rect.left = dc.rect.right - size.cx;

		dc.draw_text(text, DT_CENTER | DT_VCENTER | DT_SINGLELINE)?;

		Ok(())
	}

	fn reason(&self) -> RedrawReason {
		"Date".to_owned()
	}
}
