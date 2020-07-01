use super::Tile;

pub struct TileGrid {
	pub id: i32,
	pub visible: bool,
	pub focused_window_id: Option<i32>,
	pub taskbar_window: i32,
	pub rows: i32,
	pub columns: i32,
	pub height: i32,
	pub width: i32,
	pub tiles: Vec<Tile>,
}

impl TileGrid {
	pub fn new(id: i32) -> Self {
		Self {
			id,
			visible: false,
			focused_window_id: None,
			taskbar_window: 0,
			rows: 0,
			columns: 0,
			height: 0,
			width: 0,
			tiles: vec![],
		}
	}

	pub fn hide(&mut self) {
		warn!("TODO")
	}

	pub fn show(&mut self) {
		warn!("TODO")
	}

	pub fn draw_grid(&self) {
		warn!("TODO")
	}
}
