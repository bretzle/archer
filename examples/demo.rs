use appbar::AppBar;

fn main() {
	simple_logger::init().unwrap();

	let bar = AppBar::create();

	bar.start();

	loop {}
}
