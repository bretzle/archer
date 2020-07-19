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
static CHANNEL: OnceCell<EventChannel> = OnceCell::new();

#[derive(Debug, Default)]
pub struct AppBar {
	display: Display,
	config: Config,
	window: i32,
	font: i32,
	redraw_reason: RedrawReason,
	components: HashMap<RedrawReason, Box<dyn Component>>,
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

	pub fn start(&self) {
		thread::spawn(|| {
			let receiver = CHANNEL.get_or_init(EventChannel::default).receiver.clone();

			app_bar::create();

			loop {
				select! {
					recv(receiver) -> msg => {
						let msg = msg.unwrap();
						match msg {
							Event::RedrawAppBar(reason) => app_bar::redraw(reason),
							Event::WinEvent(_) => {
								if util::is_fullscreen() {
									app_bar::hide();
								} else {
									app_bar::show();
								}
							},
							_ => {}
						}
					}
				}
			}
		});
	}

	pub fn with_component(&'static mut self, component: Box<dyn Component>) -> &'static mut Self {
		self.components.insert(component.reason(), component);
		self
	}

	pub(crate) fn get() -> &'static mut Self {
		unsafe { INSTANCE.get_mut().unwrap() }
	}
}
