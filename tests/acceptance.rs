use polybar::prelude::PolyBar;

#[test]
fn singleton() {
	let a = PolyBar::create();
	let b = PolyBar::create();

	assert_eq!(a as *const _, b as *const _);
}
