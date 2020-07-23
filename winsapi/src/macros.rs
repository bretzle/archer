/// Converts a str to the format that the windows api uses
#[macro_export]
macro_rules! str_to_wide {
	($str:expr) => {{
		$str.encode_utf16()
			.chain(std::iter::once(0))
			.collect::<Vec<_>>()
		}};
}
