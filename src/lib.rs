use config::Config;
use crossbeam_channel::{select, SendError};
use display::Display;
use event::{Event, EventChannel};
use io::ErrorKind;
use lazy_static::lazy_static;
use std::{
	io,
	sync::{
		atomic::{AtomicBool, Ordering},
		Arc, Mutex,
	},
	thread,
};

mod app_bar;
mod config;
mod display;
mod event;
mod util;

static RUNNING: AtomicBool = AtomicBool::new(false);

lazy_static! {
	static ref CHANNEL: EventChannel = EventChannel::default();
	static ref APPBAR: Arc<Mutex<AppBar>> = Arc::new(Mutex::new(AppBar::default()));
}

#[derive(Debug, Copy, Clone, Default)]
pub struct AppBar {
	display: Display,
	config: Config,
}

impl AppBar {
	pub fn create() -> io::Result<AppBar> {
		if RUNNING.load(Ordering::SeqCst) == false {
			lazy_static::initialize(&APPBAR);

			RUNNING.store(true, Ordering::SeqCst);

			Ok(*APPBAR.clone().lock().unwrap())
		} else {
			Err(ErrorKind::AlreadyExists.into())
		}
	}

	pub fn start(&self) {
		thread::spawn(|| {
			let receiver = CHANNEL.receiver.clone();

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
}

pub fn send_message(msg: Event) -> Result<(), SendError<Event>> {
	CHANNEL.sender.send(msg)
}

fn get_config() -> Config {
	APPBAR.lock().unwrap().config
}
