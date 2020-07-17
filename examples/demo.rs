use appbar::{components::Clock, AppBar};

fn main() {
	simple_logger::init().unwrap();

	let bar = AppBar::create().with_component(Box::new(Clock::default()));

	bar.start();

	loop {}
}
