use crate::common::Rect;
use winapi::shared::windef::{HBRUSH, HDC};
use winapi::um::wingdi::{CreateSolidBrush, DeleteObject, RGB};
use winapi::um::winuser::{FillRect, FrameRect};

pub(super) const TILE_WIDTH: u32 = 48;
pub(super) const TILE_HEIGHT: u32 = 48;

#[derive(Default, Clone, Copy, PartialEq)]
pub(super) struct Tile {
	pub selected: bool,
	pub hovered: bool,
}

impl Tile {
	pub unsafe fn draw(self, hdc: HDC, area: Rect) {
		let fill_brush = self.fill_brush();
		let frame_brush = CreateSolidBrush(RGB(0, 0, 0));

		FillRect(hdc, &area.into(), fill_brush);
		FrameRect(hdc, &area.into(), frame_brush);

		DeleteObject(fill_brush as *mut _);
		DeleteObject(frame_brush as *mut _);
	}

	unsafe fn fill_brush(self) -> HBRUSH {
		let color = if self.selected {
			RGB(0, 77, 128)
		} else if self.hovered {
			RGB(0, 100, 148)
		} else {
			RGB(
				(255.0 * (70.0 / 100.0)) as u8,
				(255.0 * (70.0 / 100.0)) as u8,
				(255.0 * (70.0 / 100.0)) as u8,
			)
		};

		CreateSolidBrush(color)
	}
}
