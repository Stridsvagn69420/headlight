use std::io;
use std::fs;
use std::path::Path;
use std::fmt::Display;
use serde::Deserialize;
use toml;
use crate::meta::NAME;

const CONFIG_DIR: &str = "/etc";
const CONFIG_FILENAME: &str = "config.toml";

/// Location settings
/// 
/// Currently this sets the location manually by latitude and longitude.
#[derive(Deserialize)]
struct Location {
	/// Latitude
	lat: f32,

	/// Longitude
	lon: f32
}

/// Brightness settings
/// 
/// This set the display brightness for day and night.
/// Values have to be set as [f32].
#[derive(Deserialize)]
struct Brightness {
	day: f32,
	night: f32
}

/// Main config
#[derive(Deserialize)]
struct Config {
	location: Location,
	brightness: Brightness
}

impl Config {
	/// Load config
	/// 
	/// Attempts to load the global config file
	pub fn load(custom: Option<impl AsRef<Path>>) -> io::Result<Self> {
		// Create path
		let path = if let Some(x) = custom {
			x.as_ref().to_path_buf()
		} else {
			Path::new(CONFIG_DIR).join(NAME).join(CONFIG_FILENAME)
		};

		// Load file from path and parse as TOML
		let tomltxt = fs::read_to_string(path)?;
		toml::from_str::<Self>(&tomltxt).map_err(io::Error::other)
	}
}