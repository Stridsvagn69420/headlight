use std::{fs, io};
use std::io::{Read, Write, Seek, SeekFrom};
use std::fs::File;
use std::path::Path;
use super::BacklightController;
use atoi::atoi;
use itoa::Buffer;

pub const SYS_CLASS_BACKLIGHT: &str = "/sys/class/backlight";

/// Linux backlight class
/// 
/// This struct refers to the Linux sysfs interface at `/sys/class/backlight`.
/// It is usually used for laptop and other embedded displays.
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
}

impl BacklightController for Backlight {
	fn id(&self) -> &str {
		&self.id
	}

	fn max_brightness(&self) -> u32 {
		self.max
	}

	fn get_brightness(&mut self) -> io::Result<u32> {
		// Reset head back to start of file stream and read into buffer
		let mut buf = [0; 5];
		self.brightness.seek(SeekFrom::Start(0))?;
		let count = self.brightness.read(&mut buf)?;

		// Convert file. Unwrapping should be safe as otherwise the sysfs would be broken.
		let level = atoi(&buf[..count]).unwrap();
		Ok(level)
	}

	fn set_brightness(&mut self, level: u32) -> io::Result<()> {
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
}