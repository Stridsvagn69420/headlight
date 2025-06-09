use std::{fs, io};
use std::io::{Read, Write, Seek, SeekFrom};
use std::fs::File;
use std::path::Path;
use atoi::atoi;
use itoa::Buffer;

pub const SYS_CLASS_BACKLIGHT: &str = "/sys/class/backlight";

/// Linux backlight class
/// 
/// This struct refers to the Linux sysfs interface at `/sys/class/backlight`.
/// It is usually used for laptops and other embedded displays.
pub struct Backlight {
	/// ID
	/// 
	/// The ID represents the name of the symlink found at `/sys/class/backlight`.
	id: String,


	/// Maximum brightness
	/// 
	/// The maximum brightness read from `max_brightness`.
	max: u32,

	/// Brightness value file
	/// 
	/// The [File] handle of `brightness` to read and write the brightness value.
	brightness: File,
}

impl Backlight {
	/// New Backlight
	/// 
	/// Creates a new [Backlight] controller based on an entry inside the sysfs.
	/// - `bl`: The path to an entry inside `/sys/class/backlight`. You can get it easily from a call to [fs::read_dir].
	pub fn new(bl: impl AsRef<Path>) -> io::Result<Self> {
		// Try to correct provided path if
		let blref = bl.as_ref();
		let syspath = if blref.is_absolute() {
			blref.to_path_buf()
		} else {
			Path::new(SYS_CLASS_BACKLIGHT).join(blref)
		};

		// Read maximum brightness once
		let maxdata = fs::read(syspath.join("max_brightness"))?;
		let max = atoi(&maxdata).unwrap();

		// Open brightness file
		let brightness = File::options()
			.read(true)
			.write(true)
			.open(syspath.join("brightness"))?;

		// Convert path filename into ID string
		let Some(id) = syspath.file_name().map(|x| x.to_string_lossy().to_string()) else {
			return Err(io::Error::other("path seems to be corrupted"));
		};

		// Return created struct
		Ok(Self { id, max, brightness })
	}

	/// Controller ID
	/// 
	/// The unique identifier of a controller.
	/// It's currently the entry name in [SYS_CLASS_BACKLIGHT].
	pub fn id(&self) -> &str {
		&self.id
	}

	/// Maximum brightness
	/// 
	/// Returns the maximum brightness of the controlled display.
	pub fn max_brightness(&self) -> u32 {
		self.max
	}

	/// Get brightness
	/// 
	/// Returns the currently set brightness of the controlled display.
	pub fn get_brightness(&mut self) -> io::Result<u32> {
		// Reset head back to start of file stream and read into buffer
		let mut buf = [0; 5];
		self.brightness.seek(SeekFrom::Start(0))?;
		let count = self.brightness.read(&mut buf)?;

		// Convert file. Unwrapping should be safe as otherwise the sysfs would be broken.
		let level = atoi(&buf[..count]).unwrap();
		Ok(level)
	}

	/// Set brightness
	/// 
	/// Sets the brightness for the controlled display.
	pub fn set_brightness(&mut self, level: u32) -> io::Result<()> {
		// Abort if provided level is larger than max
		if level > self.max {
			return Err(io::Error::new(io::ErrorKind::FileTooLarge, "provided brightness higher than max"));
		}

		// Convert u32 into ASCII number
		let mut buf = Buffer::new();
		let conv = buf.format(level).as_bytes();

		// Write ASCII number into file
		self.brightness.write_all(conv)
	}

	/// Set brightness by percentage
	/// 
	/// Sets the brightness by a given percentage of the `max_brightness`.
	/// Note that this isn't the most accurate to set brightness, but the most convenient one.
	fn set_brightness_percent(&mut self, level: f32) -> io::Result<()> {
		// Checks given percentage to not continue any further if it's invalid.
		if !(0.0..=1.0).contains(&level) {
			let err = io::Error::new(io::ErrorKind::FileTooLarge, "percentage not in range of 0% and 100%");
			return Err(err);
		}
		// Calculate into u32 and run it
		let x = self.max_brightness() as f32 * level;
		self.set_brightness(x as u32)
	}
}