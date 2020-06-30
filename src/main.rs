use std::{error::Error, sync::Mutex};

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use config::Config;
use display::Display;
use event::EventChannel;
use lazy_static::lazy_static;
use workspace::*;

mod app_bar;
mod config;
mod display;
mod event;
mod logging;
mod task_bar;
mod tiles;
mod tray;
mod util;
mod workspace;

lazy_static! {
	pub static ref CONFIG: Mutex<Config> =
		Mutex::new(Config::load().expect("Failed to loading config"));
	pub static ref DISPLAY: Mutex<Display> = {
		let mut display = Display::default();
		display.init();
		Mutex::new(display)
	};
	pub static ref WORK_MODE: Mutex<bool> = Mutex::new(CONFIG.lock().unwrap().work_mode);
	pub static ref CHANNEL: EventChannel = EventChannel::default();
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

	info!("Initializing workspaces");
	lazy_static::initialize(&WORKSPACES);

	if *WORK_MODE.lock().unwrap() {
		// Work mode is enabled
		if CONFIG.lock().unwrap().remove_task_bar {
			info!("Hiding taskbar");
			task_bar::hide();
		}
	}

	workspace::change(1).expect("Failed to change to workspace @1");

	loop {}
}

fn cleanup() -> Result<(), Box<dyn Error>> {
	task_bar::show();

	Ok(())
}
