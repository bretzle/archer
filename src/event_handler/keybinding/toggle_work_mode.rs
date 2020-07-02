use crate::{app_bar, cleanup, task_bar, win_event_handler, CONFIG, WORK_MODE};
use std::sync::atomic::Ordering;

pub fn turn_work_mode_off(
	display_app_bar: bool,
	remove_task_bar: bool,
) -> Result<(), Box<dyn std::error::Error>> {
	win_event_handler::unregister()?;

	if display_app_bar {
		app_bar::close();
	}

	if remove_task_bar {
		task_bar::show();
	}

	cleanup()?;
	Ok(())
}

pub fn turn_work_mode_on(
	display_app_bar: bool,
	remove_task_bar: bool,
) -> Result<(), Box<dyn std::error::Error>> {
	win_event_handler::register()?;
	if display_app_bar {
		app_bar::create().expect("Failed to create app bar");
	}
	if remove_task_bar {
		task_bar::hide();
	}
	Ok(())
}

pub fn handle() -> Result<(), Box<dyn std::error::Error>> {
	let work_mode = WORK_MODE.load(Ordering::SeqCst);
	let display_app_bar = CONFIG.lock().unwrap().display_app_bar;
	let remove_task_bar = CONFIG.lock().unwrap().remove_task_bar;

	if work_mode {
		turn_work_mode_off(display_app_bar, remove_task_bar)?;
	} else {
		turn_work_mode_on(display_app_bar, remove_task_bar)?;
	}

	WORK_MODE.store(!work_mode, Ordering::SeqCst);

	Ok(())
}
