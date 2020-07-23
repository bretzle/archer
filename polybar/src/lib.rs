use components::Component;
use once_cell::sync::OnceCell;
use poly_bar::PolyBar;

pub mod components;
mod config;
mod display;
mod event;
mod poly_bar;
mod util;

static mut INSTANCE: OnceCell<PolyBar> = OnceCell::new();

pub mod prelude {
	pub use crate::{
		components::Component,
		event::Event,
		poly_bar::{DrawData, PolyBar, RedrawReason},
	};
}
