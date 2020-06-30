mod error;

pub use error::*;

pub fn panic_handler() {
	std::panic::set_hook(Box::new(|info| {
		// print panic message
		if let Some(s) = info.payload().downcast_ref::<&str>() {
			println!("panic occurred: {:?}", s);
		}

		// print location
		if let Some(location) = info.location() {
			println!(
				"panic occurred in file '{}' at line {}",
				location.file(),
				location.line(),
			);
		} else {
			println!("panic occurred but can't get location information...");
		}

		if let Err(e) = crate::cleanup() {
			println!("Failed to cleanup.\n{:?}", e);
		}
	}));
}

pub fn ctrlc_handler() {
	#[cfg(feature = "debug")]
	ctrlc::set_handler(|| {
		if let Err(e) = crate::cleanup() {
			error!("Something happend when cleaning up. {}", e);
		}
		std::process::exit(0);
	})
	.unwrap();
}
