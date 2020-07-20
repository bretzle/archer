use app_bar::RedrawReason;
use components::Component;
use config::Config;
use crossbeam_channel::select;
use display::Display;
use event::{Event, EventChannel};
use once_cell::sync::OnceCell;
use std::{collections::HashMap, fmt::Debug, thread};
use winsapi::Font;

mod app_bar;
pub mod components;
mod config;
mod display;
mod event;
mod util;

static mut INSTANCE: OnceCell<AppBar> = OnceCell::new();

#[derive(Debug, Default)]
pub struct AppBar {
	display: Display,
	config: Config,
	window: Option<i32>,
	font: Font,
	redraw_reason: RedrawReason,
	components: HashMap<RedrawReason, Box<dyn Component>>,
	channel: EventChannel,
	draw_data: Option<DrawData>,
}

impl AppBar {
	pub fn create() -> &'static mut Self {
		unsafe {
			match INSTANCE.get_mut() {
				Some(instance) => instance,
				None => {
					INSTANCE.set(AppBar::default()).unwrap();
					INSTANCE.get_mut().unwrap()
				}
			}
		}
	}

	pub fn with_component(&'static mut self, component: Box<dyn Component>) -> &'static mut Self {
		if self
			.components
			.insert(component.reason(), component)
			.is_some()
		{
			panic!("Two components can not have the same reason");
		}
		self
	}

	pub fn start(&'static self) {
		thread::spawn(move || {
			let receiver = self.channel.receiver.clone();

			app_bar::create();

			loop {
				select! {
					recv(receiver) -> msg => {
						Self::handle_event(msg.unwrap());
					}
				}
			}
		});
	}

	fn handle_event(msg: Event) {
		match msg {
			Event::RedrawAppBar(reason) => app_bar::redraw(reason).unwrap(),
			Event::WinEvent(_) => {
				if util::is_fullscreen() {
					app_bar::hide();
				} else {
					app_bar::show();
				}
			}
			_ => {}
		}
	}

	fn init_draw_data(&'static mut self) {
		self.draw_data = Some(DrawData {
			display: &self.display,
			bg_color: &self.config.bg_color,
			font: &self.font,
		})
	}
}

pub mod prelude {
	pub use crate::{
		app_bar::RedrawReason,
		components::Component,
		event::{Event, EventSender},
		AppBar, DrawData,
	};
}

#[derive(Debug)]
pub struct DrawData {
	pub display: &'static Display,
	pub bg_color: &'static i32,
	pub font: &'static Font,
}
