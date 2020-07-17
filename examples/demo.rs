use appbar::AppBar;

fn main() {
	simple_logger::init().unwrap();

	let bar = AppBar::create().unwrap();

	bar.start();

	loop {}
}
