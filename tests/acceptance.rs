use appbar::AppBar;

#[test]
fn singleton() {
	let a = AppBar::create();
	let b = AppBar::create();

	assert_eq!(a as *const _, b as *const _);
}
