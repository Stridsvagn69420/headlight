use std::io;
use super::BacklightController;

/// Linux backlight class
/// 
/// This struct refers to the Linux sysfs interface at `/sys/class/backlight`.
/// It is usually used for laptop and other embedded displays.
pub struct Backlight {
	/// ID
	/// 
	/// The ID represents the name of the symlink found at `/sys/class/backlight`.
	id: String,
}

impl BacklightController for Backlight {
	fn id(&self) -> &str {
		&self.id
	}

	fn max_brightness(&self) -> u32 {
		todo!()
	}

	fn get_brightness(&self) -> io::Result<u32> {
		todo!()
	}

	fn set_brightness(&self, level: u32) -> io::Result<()> {
		todo!()
	}
}