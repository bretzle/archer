use std::{error::Error, sync::Mutex};

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use config::Config;
use display::Display;
use lazy_static::lazy_static;

mod config;
mod display;
mod logging;
mod task_bar;
mod tray;
mod util;

lazy_static! {
	pub static ref CONFIG: Mutex<Config> =
		Mutex::new(Config::load().expect("Failed to loading config"));
	pub static ref DISPLAY: Mutex<Display> = {
		let mut display = Display::default();
		display.init();
		Mutex::new(display)
	};
}

fn main() {
	logging::setup();

	util::panic_handler();
	util::ctrlc_handler();

	if let Err(e) = run() {
		error!("An error occured {:?}", e);
		if let Err(e) = cleanup() {
			error!("Something happend when cleaning up. {}", e);
		}
	}
}

fn run() -> Result<(), Box<dyn Error>> {
	info!("Initializing config");
	lazy_static::initialize(&CONFIG);

	info!("Initializing display");
	lazy_static::initialize(&DISPLAY);

	info!("Initializing taskbar");
	task_bar::init();

	info!("Creating tray icon");
	tray::create()?;

	loop {}
}

fn cleanup() -> Result<(), Box<dyn Error>> {
	Ok(())
}
