use std::io;

mod backlight;
pub use backlight::*;

/// Backlight Controller trait
/// 
/// Trait providing all neccesary controls to read and
/// modify display backlight. 
pub trait BacklightController {
	/// Controller ID
	/// 
	/// The unique identifier of a controller.
	fn id(&self) -> &str;

	/// Maximum brightness
	/// 
	/// Returns the maximum brightness of the controlled display.
	fn max_brightness(&self) -> u32;

	/// Current brightness
	/// 
	/// Returns the currently set brightness of the controlled display.
	fn get_brightness(&self) -> io::Result<u32>;

	/// Set brightness
	/// 
	/// Sets the brightness for the controlled display.
	fn set_brightness(&self, level: u32) -> io::Result<()>;

	/// Set brightness by percentage
	/// 
	/// Sets the brightness by a given percentage of the `max_brightness()`.
	/// Note that this isn't the most accurate to set brightness, but the most convenient.
	fn set_brightness_percent(&self, level: f32) -> io::Result<()> {
		// Checks given percentage to not continue any further if it's invalid.
		if !(0.0..=1.0).contains(&level) {
			let err = io::Error::other("percentage not in range of 0% and 100%");
			return Err(err);
		}
		
		// Calculate into u32 and run it
		let x = self.max_brightness() as f32 * level;
		self.set_brightness(x as u32)
	}
}