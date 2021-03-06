//! Grid module

mod tile;

use std::mem;
use tile::*;
use winapi::um::winuser::{BeginPaint, EndPaint, PAINTSTRUCT};
use winsapi::{Monitor, Rect, Window};

//TODO document this better
/// The grid!
#[derive(Debug)]
pub struct Grid {
	/// is the Shift key down
	pub shift_down: bool,
	/// is the Control key down
	pub control_down: bool,
	/// is the mouse down
	pub cursor_down: bool,
	/// The selected tile
	pub selected_tile: Option<(usize, usize)>,
	/// The tile the mouse is hovering over
	pub hovered_tile: Option<(usize, usize)>,
	/// The active window
	pub active_window: Option<Window>,
	/// The grid's window that the grid will be drawn on
	pub grid_window: Option<Window>,
	/// The last resize operation
	pub previous_resize: Option<(Window, Rect)>,
	/// is quick resize being used
	pub quick_resize: bool,
	grid_margins: u8,
	zone_margins: u8,
	border_margins: u8,
	tiles: Vec<Vec<Tile>>, // tiles[row][column]
}

impl Grid {
	/// Resets the grid
	pub fn reset(&mut self) {
		self.shift_down = false;
		self.control_down = false;
		self.cursor_down = false;
		self.selected_tile = None;
		self.hovered_tile = None;
		self.grid_window = None;
		self.quick_resize = false;

		self.tiles.iter_mut().for_each(|row| {
			row.iter_mut().for_each(|tile| {
				tile.selected = false;
				tile.hovered = false;
			})
		});
	}

	/// Get the dimensions of the grid window
	pub fn dimensions(&self) -> (u32, u32) {
		let width = self.columns() as u32 * TILE_WIDTH
			+ (self.columns() as u32 + 1) * self.grid_margins as u32;

		let height =
			self.rows() as u32 * TILE_HEIGHT + (self.rows() as u32 + 1) * self.grid_margins as u32;

		(width, height)
	}

	fn zone_area(&self, row: usize, column: usize) -> Rect {
		let work_area = Monitor::get_active().area();

		let zone_width = (work_area.w
			- self.border_margins as i32 * 2
			- (self.columns() - 1) as i32 * self.zone_margins as i32)
			/ self.columns() as i32;
		let zone_height = (work_area.h
			- self.border_margins as i32 * 2
			- (self.rows() - 1) as i32 * self.zone_margins as i32)
			/ self.rows() as i32;

		let x = column as i32 * zone_width
			+ self.border_margins as i32
			+ column as i32 * self.zone_margins as i32
			+ work_area.x;
		let y = row as i32 * zone_height
			+ self.border_margins as i32
			+ row as i32 * self.zone_margins as i32
			+ work_area.y;

		Rect {
			x,
			y,
			w: zone_width,
			h: zone_height,
		}
	}

	fn rows(&self) -> usize {
		self.tiles.len()
	}

	fn columns(&self) -> usize {
		self.tiles[0].len()
	}

	/// Adds a row to the grid
	pub fn add_row(&mut self) {
		self.tiles.push(vec![Tile::default(); self.columns()]);
	}

	/// Adds a column to the grid
	pub fn add_column(&mut self) {
		for row in self.tiles.iter_mut() {
			row.push(Tile::default());
		}
	}

	/// Removes a row from the grid
	pub fn remove_row(&mut self) {
		if self.rows() > 1 {
			self.tiles.pop();
		}
	}

	/// Removes a column from the grid
	pub fn remove_column(&mut self) {
		if self.columns() > 1 {
			for row in self.tiles.iter_mut() {
				row.pop();
			}
		}
	}

	fn tile_area(&self, row: usize, column: usize) -> Rect {
		let x = column as i32 * TILE_WIDTH as i32 + (column as i32 + 1) * self.grid_margins as i32;

		let y = row as i32 * TILE_HEIGHT as i32 + (row as i32 + 1) * self.grid_margins as i32;

		Rect {
			x,
			y,
			w: TILE_WIDTH as i32,
			h: TILE_HEIGHT as i32,
		}
	}

	/// Recenters the grid window after the a new row or column is added
	pub fn reposition(&mut self) {
		let work_area = Monitor::get_active().area();
		let dimensions = self.dimensions();

		let rect = Rect {
			x: work_area.w / 2 - dimensions.0 as i32 / 2 + work_area.x,
			y: work_area.h / 2 - dimensions.1 as i32 / 2 + work_area.y,
			w: dimensions.0 as i32,
			h: dimensions.1 as i32,
		};

		self.grid_window.as_mut().unwrap().set_pos(rect, None);
	}

	/// Returns true if a change in highlighting occured
	pub unsafe fn highlight_tiles(&mut self, point: (i32, i32)) -> Option<Rect> {
		let original_tiles = self.tiles.clone();
		let mut hovered_rect = None;

		for row in 0..self.rows() {
			for column in 0..self.columns() {
				let tile_area = self.tile_area(row, column);

				if tile_area.contains_point(point) {
					self.tiles[row][column].hovered = true;

					self.hovered_tile = Some((row, column));
					hovered_rect = Some(self.zone_area(row, column));
				} else {
					self.tiles[row][column].hovered = false;
				}
			}
		}

		if let Some(rect) = self.shift_hover_and_calc_rect(true) {
			hovered_rect = Some(rect);
		}

		if original_tiles == self.tiles {
			None
		} else {
			hovered_rect
		}
	}

	unsafe fn shift_hover_and_calc_rect(&mut self, highlight: bool) -> Option<Rect> {
		if self.shift_down || self.cursor_down {
			if let Some(selected_tile) = self.selected_tile {
				if let Some(hovered_tile) = self.hovered_tile {
					let selected_zone = self.zone_area(selected_tile.0, selected_tile.1);
					let hovered_zone = self.zone_area(hovered_tile.0, hovered_tile.1);

					let from_tile;
					let to_tile;

					let hovered_rect = if hovered_zone.x < selected_zone.x
						&& hovered_zone.y > selected_zone.y
					{
						from_tile = (selected_tile.0, hovered_tile.1);
						to_tile = (hovered_tile.0, selected_tile.1);

						let from_zone = self.zone_area(from_tile.0, from_tile.1);
						let to_zone = self.zone_area(to_tile.0, to_tile.1);

						Rect {
							x: from_zone.x,
							y: from_zone.y,
							w: (to_zone.x + to_zone.w) - from_zone.x,
							h: (to_zone.y + to_zone.h) - from_zone.y,
						}
					} else if hovered_zone.y < selected_zone.y && hovered_zone.x > selected_zone.x {
						from_tile = (hovered_tile.0, selected_tile.1);
						to_tile = (selected_tile.0, hovered_tile.1);

						let from_zone = self.zone_area(from_tile.0, from_tile.1);
						let to_zone = self.zone_area(to_tile.0, to_tile.1);

						Rect {
							x: from_zone.x,
							y: from_zone.y,
							w: (to_zone.x + to_zone.w) - from_zone.x,
							h: (to_zone.y + to_zone.h) - from_zone.y,
						}
					} else if hovered_zone.x > selected_zone.x || hovered_zone.y > selected_zone.y {
						from_tile = selected_tile;
						to_tile = hovered_tile;

						Rect {
							x: selected_zone.x,
							y: selected_zone.y,
							w: (hovered_zone.x + hovered_zone.w) - selected_zone.x,
							h: (hovered_zone.y + hovered_zone.h) - selected_zone.y,
						}
					} else {
						from_tile = hovered_tile;
						to_tile = selected_tile;

						Rect {
							x: hovered_zone.x,
							y: hovered_zone.y,
							w: (selected_zone.x + selected_zone.w) - hovered_zone.x,
							h: (selected_zone.y + selected_zone.h) - hovered_zone.y,
						}
					};

					if highlight {
						for row in from_tile.0..=to_tile.0 {
							for column in from_tile.1..=to_tile.1 {
								self.tiles[row][column].hovered = true;
							}
						}
					}

					return Some(hovered_rect);
				}
			}
		}

		None
	}

	/// Selects a tile
	pub unsafe fn select_tile(&mut self, point: (i32, i32)) -> bool {
		if self.cursor_down || self.shift_down {
			return false;
		}

		let previously_selected = self.selected_tile;

		for row in 0..self.rows() {
			for column in 0..self.columns() {
				let tile_area = self.tile_area(row, column);

				if tile_area.contains_point(point) {
					self.tiles[row][column].selected = true;

					self.selected_tile = Some((row, column));
				} else {
					self.tiles[row][column].selected = false;
				}
			}
		}

		self.selected_tile != previously_selected
	}

	/// Gets the selected area
	pub unsafe fn selected_area(&mut self) -> Option<Rect> {
		if let Some(shift_rect) = self.shift_hover_and_calc_rect(false) {
			return Some(shift_rect);
		}

		if let Some(selected_tile) = self.selected_tile {
			Some(self.zone_area(selected_tile.0, selected_tile.1))
		} else {
			None
		}
	}

	/// Unhighlights all tiles
	pub fn unhighlight_all_tiles(&mut self) {
		self.tiles
			.iter_mut()
			.for_each(|row| row.iter_mut().for_each(|tile| tile.hovered = false));
	}

	/// Unselects all tiles
	pub fn unselect_all_tiles(&mut self) {
		self.tiles
			.iter_mut()
			.for_each(|row| row.iter_mut().for_each(|tile| tile.selected = false));
	}

	/// Draws the grid to the window
	pub unsafe fn draw(&self, window: Window) {
		let mut paint: PAINTSTRUCT = mem::zeroed();
		//paint.fErase = 1;

		let hdc = BeginPaint(window.0, &mut paint);

		for row in 0..self.rows() {
			for column in 0..self.columns() {
				self.tiles[row][column].draw(hdc, self.tile_area(row, column));
			}
		}

		EndPaint(window.0, &paint);
	}
}

impl Default for Grid {
	fn default() -> Self {
		let rows = 2;
		let columns = 2;

		Grid {
			shift_down: false,
			control_down: false,
			cursor_down: false,
			selected_tile: None,
			hovered_tile: None,
			active_window: None,
			grid_window: None,
			previous_resize: None,
			quick_resize: false,
			grid_margins: 3,
			zone_margins: 10,
			border_margins: 10,
			tiles: vec![vec![Tile::default(); columns]; rows],
		}
	}
}
