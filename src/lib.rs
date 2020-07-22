mod config;
mod wtm;

use once_cell::sync::OnceCell;
use wtm::WTM;

static mut INSTANCE: OnceCell<WTM> = OnceCell::new();

pub mod prelude {
	pub use crate::wtm::WTM;
}
