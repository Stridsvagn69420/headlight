use std::error::Error;
use std::fs;
use std::thread::sleep;
use std::time::Duration;

mod config;
mod meta;
mod interfaces;
use interfaces::{Backlight, BacklightController, SYS_CLASS_BACKLIGHT};

fn main() -> Result<(), Box<dyn Error>> {
	// NOTE: Everything in main is currently just a test!!!

	// Read sysfs directory
	let direntries: Vec<_> = fs::read_dir(SYS_CLASS_BACKLIGHT)?.filter_map(|x| x.ok()).collect();
	if direntries.is_empty() {
		println!("No backlight control paths found!");
		return Ok(());
	}

	// Attempt to create Backlight controller struct
	let mut controls: Vec<Backlight> = Vec::new();
	for entry in direntries {
		match Backlight::new(entry.path()) {
			Ok(ctrl) => {
				let id = ctrl.id();
				println!("[{id}] Successfully created Backlight controller.");
				println!("[{id}] Maximum brightness: {}", ctrl.max_brightness());
				controls.push(ctrl);
			},
			Err(err) => {
				println!("Failed to create Backlight controller for {}", entry.path().to_string_lossy());
				return Err(Box::new(err));
			}
		}
	}

	// Display current brightness for every available controller
	loop {
		for ctrl in &mut controls {
			// Display stats
			let Ok(light) = ctrl.get_brightness() else {
				println!("[{}] Failed to read brightness!", ctrl.id());
				continue;
			};
			let percentage = light as f32 / ctrl.max_brightness() as f32 * 100.0;
			println!("[{}] Current brightness: {}/{} ({percentage}%)", ctrl.id(), light, ctrl.max_brightness());
		}
		sleep(Duration::from_secs(1));
	}
}