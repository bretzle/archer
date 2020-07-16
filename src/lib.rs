use config::Config;
use crossbeam_channel::{select, SendError};
use display::Display;
use event::{Event, EventChannel};
use lazy_static::lazy_static;
use log::info;
use std::{error::Error, sync::Mutex};

mod app_bar;
mod config;
mod display;
mod event;
mod util;

lazy_static! {
	pub static ref CONFIG: Mutex<Config> = Mutex::new(Config::default());
	pub static ref DISPLAY: Mutex<Display> = Mutex::new(Display::new());
	pub static ref CHANNEL: EventChannel = EventChannel::default();
}

pub fn run() -> Result<(), Box<dyn Error>> {
	let receiver = CHANNEL.receiver.clone();

	info!("Initializing config");
	lazy_static::initialize(&CONFIG);

	info!("Initializing display");
	lazy_static::initialize(&DISPLAY);

	app_bar::create(&Display::new())?;

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
}

pub fn send_message(msg: Event) -> Result<(), SendError<Event>> {
	CHANNEL.sender.send(msg)
}
