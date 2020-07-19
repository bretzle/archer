use app_bar::RedrawReason;
use components::Component;
use config::Config;
use crossbeam_channel::select;
use display::Display;
use event::{Event, EventChannel};
use once_cell::sync::OnceCell;
use std::{collections::HashMap, fmt::Debug, thread};

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
	font: i32,
	redraw_reason: RedrawReason,
	components: HashMap<RedrawReason, Box<dyn Component>>,
	channel: EventChannel,
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
			Event::RedrawAppBar(reason) => app_bar::redraw(reason),
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
}

pub mod prelude {
	pub use crate::{
		app_bar::{set_font, RedrawReason},
		components::Component,
		event::{Event, EventSender},
		util::{CTypeExt, PtrExt, WinApiError},
		AppBar,
	};

	pub use winapi::shared::windef::HWND;
}
