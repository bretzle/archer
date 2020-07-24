use polybar::components::{Clock, Date};
use polybar::prelude::*;
use wtm::TilingManager;

fn main() {
	simple_logger::init().unwrap();

	let tm = TilingManager::create();
	let bar = PolyBar::create()
		.with_component(Box::new(Clock::default()))
		.with_component(Box::new(Date::default()));

	tm.start();
	bar.start();

	loop {}
}
