use winapi::{
	shared::windef::RECT,
	um::winuser::{GetDesktopWindow, GetForegroundWindow, GetWindowRect},
};

pub fn is_fullscreen() -> bool {
	let mut a = RECT::default();
	let mut b = RECT::default();

	unsafe {
		let hwnd = GetForegroundWindow();
		GetWindowRect(hwnd, &mut a);
		GetWindowRect(GetDesktopWindow(), &mut b);
	}

	a.left == b.left && a.top == b.top && a.right == b.right && a.bottom == b.bottom
}
