use crate::{
	poly_bar::{DrawData, RedrawReason},
	Component,
};
use std::time::Duration;
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

		dc.draw_text(text, TextOptions::default())?;

		Ok(())
	}

	fn reason(&self) -> RedrawReason {
		"Date".to_owned()
	}
}
