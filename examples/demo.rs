use appbar::{components::*, AppBar};
use std::{thread, time::Duration};

fn main() {
	simple_logger::init().unwrap();

	let bar = AppBar::create()
		.with_component(Box::new(Clock::default()))
		.with_component(Box::new(Date::default()));

	bar.start();

	loop {
		thread::sleep(Duration::from_millis(1000));
	}
}
