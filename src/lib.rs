use app_bar::AppBar;
use components::Component;
use once_cell::sync::OnceCell;

mod app_bar;
pub mod components;
mod config;
mod display;
mod event;
mod util;

static mut INSTANCE: OnceCell<AppBar> = OnceCell::new();

pub mod prelude {
	pub use crate::{
		app_bar::{AppBar, DrawData, RedrawReason},
		components::Component,
		event::{Event, EventSender},
	};
}
