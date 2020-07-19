use appbar::{components::Clock, AppBar};
use std::{thread, time::Duration};

fn main() {
	simple_logger::init().unwrap();

	let bar = AppBar::create().with_component(Box::new(Clock::default()));

	bar.start();

	loop {
		thread::sleep(Duration::from_millis(1000));
	}
}
