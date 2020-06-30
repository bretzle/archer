use crate::{
	app_bar::RedrawAppBarReason, event::Event, tiles::TileGrid, util::WinError, CHANNEL, CONFIG,
	DISPLAY,
};
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
	pub static ref WORKSPACE_ID: Mutex<i32> = Mutex::new(1);
	pub static ref WORKSPACES: Mutex<Vec<Workspace>> =
		Mutex::new((1..=10).map(Workspace::new).collect());
	pub static ref GRIDS: Mutex<Vec<TileGrid>> = {
		Mutex::new(
			(1..11)
				.map(|i| {
					let mut grid = TileGrid::new(i);
					let config = CONFIG.lock().unwrap();

					grid.height =
						DISPLAY.lock().unwrap().height - config.margin * 2 - config.padding * 2;
					grid.width =
						DISPLAY.lock().unwrap().width - config.margin * 2 - config.padding * 2;

					if config.display_app_bar {
						grid.height -= config.app_bar_height;
					}

					grid
				})
				.collect(),
		)
	};
}

pub struct Workspace {
	pub id: i32,
	pub visible: bool,
}

impl Workspace {
	pub fn new(id: i32) -> Self {
		Self { id, visible: false }
	}
}

pub fn change(id: i32) -> Result<(), WinError> {
	let mut grids = GRIDS.lock().unwrap();
	let mut gid = WORKSPACE_ID.lock().unwrap();

	let old_id = *gid;
	*gid = id;

	let mut grid = grids.iter_mut().find(|g| g.id == *gid).unwrap();
	grid.visible = true;

	if old_id == id {
		debug!("Workspace is already selected");
		return Ok(());
	}

	debug!("Showing the next workspace");
	grid.visible = true;
	grid.draw_grid();
	grid.show();

	//without this delay there is a slight flickering of the background
	std::thread::sleep(std::time::Duration::from_millis(5));

	debug!("Hiding the current workspace");
	let mut grid = grids.iter_mut().find(|g| g.id == old_id).unwrap();
	grid.visible = false;
	grid.hide();

	drop(grids);
	drop(gid);

	CHANNEL
		.sender
		.clone()
		.send(Event::RedrawAppBar(RedrawAppBarReason::Workspace))
		.expect("Failed to send redraw-app-bar event");

	Ok(())
}
