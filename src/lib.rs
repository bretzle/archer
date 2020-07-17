use components::Component;
use config::Config;
use crossbeam_channel::{select, SendError};
use display::Display;
use event::{Event, EventChannel};
use once_cell::sync::OnceCell;
use std::{fmt::Debug, thread};

mod app_bar;
mod components;
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
	components: Vec<Box<dyn Component>>,
}

impl AppBar {
	pub fn create() -> &'static mut AppBar {
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
						if let Event::RedrawAppBar(reason) = msg.unwrap() {
							app_bar::redraw(reason);
						}
					}
				}
			}
		});
	}

	pub(crate) fn config() -> Config {
		unsafe { INSTANCE.get().unwrap().config }
	}

	pub(crate) fn get() -> &'static Self {
		unsafe { INSTANCE.get_unchecked() }
	}

	pub(crate) fn send_message(msg: Event) -> Result<(), SendError<Event>> {
		CHANNEL.get().unwrap().sender.send(msg)
	}
}
