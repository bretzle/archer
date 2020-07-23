use crate::poly_bar::{DrawData, RedrawReason};
use std::{fmt::Debug, time::Duration};
use winsapi::{DeviceContext, WinApiResult};

mod clock;
mod date;

pub use clock::Clock;
pub use date::Date;

#[allow(unused_variables, unused_mut)]
pub trait Component: Debug + Send + Sync {
	fn interval(&self) -> Duration;
	fn draw(&self, data: &DrawData, mut dc: DeviceContext) -> WinApiResult<()> {
		panic!("No Draw implementation")
	}
	fn reason(&self) -> RedrawReason;
}
