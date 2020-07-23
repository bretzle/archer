use crate::{error_handler::PtrExt, WinApiResult};
use std::ffi::CString;
use winapi::um::{
	wingdi::{CreateFontIndirectA, LOGFONTA},
	winuser::{DT_CENTER, DT_SINGLELINE, DT_VCENTER},
};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Font(pub(crate) i32);

impl Font {
	pub fn create(name: &str, size: i32) -> WinApiResult<Self> {
		let mut logfont = LOGFONTA::default();
		let mut font_name: [i8; 32] = [0; 32];

		for (i, byte) in CString::new(name).unwrap().as_bytes().iter().enumerate() {
			font_name[i] = *byte as i8;
		}

		logfont.lfHeight = size;
		logfont.lfFaceName = font_name;

		let inner = unsafe { CreateFontIndirectA(&logfont).as_result()? as i32 };

		Ok(Self(inner))
	}

	pub fn to_inner(self) -> i32 {
		self.0
	}
}

pub struct TextOptions(pub(crate) u32);

impl TextOptions {
	pub fn new() -> Self {
		Self(0)
	}

	pub fn hcenter(mut self) -> Self {
		self.0 |= DT_CENTER;
		self
	}

	pub fn vcenter(mut self) -> Self {
		self.0 |= DT_VCENTER;
		self
	}

	pub fn single_line(mut self) -> Self {
		self.0 |= DT_SINGLELINE;
		self
	}
}

impl Default for TextOptions {
	fn default() -> Self {
		Self::new().hcenter().vcenter().single_line()
	}
}
