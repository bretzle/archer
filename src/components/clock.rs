use crate::{
	app_bar::{DrawData, RedrawReason},
	Component,
};
use std::time::Duration;
use winsapi::*;

#[derive(Debug, Default)]
pub struct Clock {}

impl Component for Clock {
	fn interval(&self) -> Duration {
		Duration::from_millis(950)
	}

	fn draw(&self, data: &DrawData, mut dc: DeviceContext) -> WinApiResult<()> {
		let text = format!("{}", chrono::Local::now().format("%T"));

		dc.set_font(*data.font);
		let size = dc.get_text_extent(text.clone())?;

		dc.rect.left = data.display.width / 2 - (size.cx / 2) - 10;
		dc.rect.right = data.display.width / 2 + (size.cx / 2) + 10;

		dc.set_text_color(0x00ffffff);
		dc.set_background_color(*data.bg_color as u32);

		dc.draw_text(text, TextOptions::default())?;

		Ok(())
	}

	fn reason(&self) -> RedrawReason {
		"Time".to_owned()
	}
}
