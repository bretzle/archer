use crate::str_to_wide;
use std::{env, fs, mem, ptr};
use winapi::{
	shared::minwindef::HKEY,
	um::{
		winnt::{KEY_SET_VALUE, REG_OPTION_NON_VOLATILE, REG_SZ},
		winreg::{RegCreateKeyExW, RegDeleteKeyValueW, RegSetValueExW, HKEY_CURRENT_USER},
	},
};

pub unsafe fn toggle_autostart_registry_key(enabled: bool) {
	if let Some(mut app_path) = dirs::config_dir() {
		app_path.push(".wtm");
		app_path.push("wtm.exe");

		if let Ok(current_path) = env::current_exe() {
			if current_path != app_path && enabled {
				let _ = fs::copy(current_path, &app_path);
			}

			let app_path = str_to_wide!(app_path.to_str().unwrap_or_default());
			let mut key_name = str_to_wide!("Software\\Microsoft\\Windows\\CurrentVersion\\Run");
			let mut value_name = str_to_wide!("wtm");

			let mut key: HKEY = mem::zeroed();

			if enabled {
				if RegCreateKeyExW(
					HKEY_CURRENT_USER,
					key_name.as_mut_ptr(),
					0,
					ptr::null_mut(),
					REG_OPTION_NON_VOLATILE,
					KEY_SET_VALUE,
					ptr::null_mut(),
					&mut key,
					ptr::null_mut(),
				) == 0
				{
					RegSetValueExW(
						key,
						value_name.as_mut_ptr(),
						0,
						REG_SZ,
						app_path.as_ptr() as _,
						app_path.len() as u32 * 2,
					);
				};
			} else {
				RegDeleteKeyValueW(
					HKEY_CURRENT_USER,
					key_name.as_mut_ptr(),
					value_name.as_mut_ptr(),
				);
			}
		}
	}
}
