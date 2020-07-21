use poly_bar::PolyBar;
use components::Component;
use once_cell::sync::OnceCell;

mod poly_bar;
pub mod components;
mod config;
mod display;
mod event;
mod util;

static mut INSTANCE: OnceCell<PolyBar> = OnceCell::new();

pub mod prelude {
	pub use crate::{
		poly_bar::{PolyBar, DrawData, RedrawReason},
		components::Component,
		event::{Event, EventSender},
	};
}
