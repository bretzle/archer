use std::fmt::{Display, Error, Formatter};
use winapi::shared::windef::RECT;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
	pub x: i32,
	pub y: i32,
	pub w: i32,
	pub h: i32,
}

impl Rect {
	/// Determines if a point is inside of the rectangle
	pub fn contains_point(self, point: (i32, i32)) -> bool {
		point.0 >= self.x
			&& point.0 <= self.x + self.w
			&& point.1 >= self.y
			&& point.1 <= self.y + self.h
	}

	/// Creates a new `Rect` with all values of zero
	pub fn zero() -> Self {
		Rect {
			x: 0,
			y: 0,
			w: 0,
			h: 0,
		}
	}

	/// Adjusts the rectangle for a border
	pub fn adjust_for_border(&mut self, border: (i32, i32)) {
		self.x -= border.0;
		self.w += border.0 * 2;
		self.h += border.1;
	}
}

impl Display for Rect {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
		writeln!(f, "x: {}", self.x)?;
		writeln!(f, "y: {}", self.y)?;
		writeln!(f, "w: {}", self.w)?;
		writeln!(f, "h: {}", self.h)?;

		Ok(())
	}
}

impl From<RECT> for Rect {
	fn from(rect: RECT) -> Self {
		Rect {
			x: rect.left,
			y: rect.top,
			w: rect.right - rect.left,
			h: rect.bottom - rect.top,
		}
	}
}

impl From<Rect> for RECT {
	fn from(rect: Rect) -> Self {
		RECT {
			left: rect.x,
			top: rect.y,
			right: rect.x + rect.w,
			bottom: rect.y + rect.h,
		}
	}
}
