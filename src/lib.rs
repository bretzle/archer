use config::Config;
use crossbeam_channel::{select, SendError};
use display::Display;
use event::{Event, EventChannel};
use once_cell::sync::OnceCell;
use std::thread;
use app_bar::load_font;

mod app_bar;
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
			let receiver = CHANNEL
				.get_or_init(|| EventChannel::default())
				.receiver
				.clone();

			app_bar::create(&Display::default());

			loop {
				select! {
					recv(receiver) -> maybe_msg => {
						let msg = maybe_msg.unwrap();
						match msg {
							Event::RedrawAppBar(reason) => app_bar::redraw(reason),
							_ => {}
						}
					}
				}
			}
		});
	}

	pub(crate) fn config() -> Config {
		unsafe { INSTANCE.get().unwrap().config }
	}

	pub(crate) fn send_message(msg: Event) -> Result<(), SendError<Event>> {
		CHANNEL.get().unwrap().sender.send(msg)
	}
}
