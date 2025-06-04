use std::error::Error;
use std::fs;
use std::thread::sleep;
use std::time::Duration;
use sunrise::{Coordinates, SolarDay, SolarEvent, DawnType};
use chrono::Local;

mod config;
mod meta;
mod interfaces;
use interfaces::{Backlight, BacklightController, SYS_CLASS_BACKLIGHT};

fn main() -> Result<(), Box<dyn Error>> {
	// NOTE: Everything in main is currently just a test!!!


	// Sunrise setup
	let coords = Coordinates::new(50.88, 7.12).unwrap();
	let date = Local::now();
	let solday = SolarDay::new(coords, date.date_naive());

	// Calculate every event in UTC
	let sunrise = solday.event_time(SolarEvent::Sunrise);
	let sunset = solday.event_time(SolarEvent::Sunset);
	let dawn_civil = solday.event_time(SolarEvent::Dawn(DawnType::Civil));
	let dawn_nauti = solday.event_time(SolarEvent::Dawn(DawnType::Nautical));
	let dusk_civil = solday.event_time(SolarEvent::Dusk(DawnType::Civil));
	let dusk_nauti = solday.event_time(SolarEvent::Dusk(DawnType::Nautical));

	// Convert it to Local time
	let local_sunrise = sunrise.with_timezone(&Local);
	let local_sunset = sunset.with_timezone(&Local);
	let local_dawn_civil = dawn_civil.with_timezone(&Local);
	let local_dawn_nauti = dawn_nauti.with_timezone(&Local);
	let local_dusk_civil = dusk_civil.with_timezone(&Local);
	let local_dusk_nauti = dusk_nauti.with_timezone(&Local);

	// Print UTC and Local times
	println!("-------------------[UTC]-------------------");
	println!("[Horizon]  Sunrise: {} | Sunset: {}", sunrise, sunset);
	println!("[Civil]    Dawn:    {} | Dusk    {}", dawn_civil, dusk_civil);
	println!("[Nautical] Dawn:    {} | Dusk    {}", dawn_nauti, dusk_nauti);
	println!("------------------[LOCAL]------------------");
	println!("[Horizon]  Sunrise: {} | Sunset: {}", local_sunrise, local_sunset);
	println!("[Civil]    Dawn:    {} | Dusk    {}", local_dawn_civil, local_dusk_civil);
	println!("[Nautical] Dawn:    {} | Dusk    {}", local_dawn_nauti, local_dusk_nauti);


	// Read sysfs directory
	let direntries: Vec<_> = fs::read_dir(SYS_CLASS_BACKLIGHT)?.filter_map(|x| x.ok()).collect();
	if direntries.is_empty() {
		println!("No valid backlight control paths found!");
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

	loop {
		// Display current brightness for every available controller
		for ctrl in &mut controls {
			// Display stats
			let Ok(light) = ctrl.get_brightness() else {
				println!("[{}] Failed to read brightness!", ctrl.id());
				continue;
			};
			let percentage = light as f32 / ctrl.max_brightness() as f32 * 100.0;
			println!("[{}] Current brightness: {}/{} ({percentage}%)", ctrl.id(), light, ctrl.max_brightness());
		}
		sleep(Duration::from_secs(2));
	}
}