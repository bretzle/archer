use appbar::AppBar;

#[test]
fn singleton() {
	assert!(AppBar::create().is_ok());
	assert!(AppBar::create().is_err());
}