use std::error::Error;
use std::fs;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use atoi::atoi;

mod interfaces;

fn main() -> Result<(), Box<dyn Error>> {
	// Open backlight for testing
	let intel_bl = Path::new("/sys/class/backlight/intel_backlight");
	let mut bf = fs::File::open(intel_bl.join("brightness"))?;

	let mut buff = [0; 5];
	loop {
		// Reset position to start and read
		bf.seek(SeekFrom::Start(0))?;
		bf.read_exact(&mut buff)?;

		// Conver to integer
		let blval = atoi::<u32>(&buff).unwrap();

		// Print value
		println!("Brightness value: {blval}");
		sleep(Duration::from_secs(1));
	}
}