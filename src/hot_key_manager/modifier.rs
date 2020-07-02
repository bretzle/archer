// use bitflags::bitflags;

// bitflags! {
// 	#[derive(Default)]
// 	#[allow(dead_code)]
// 	pub struct Modifier: u32 {
// 		const ALT = 0x0001;
// 		const CONTROL = 0x0002;
// 		const SHIFT = 0x0004;
// 	}
// }

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[repr(u32)]
pub enum Modifier {
	Alt = 1,
	Control = 2,
	Shift = 4,
	AltControl = 3,
	AltShift = 5,
	AltControlShift = 7,
}
