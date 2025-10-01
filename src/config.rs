use std::io;
use std::fs;
use std::path::Path;
use std::default::Default;
use std::fmt::Display;
use serde::Deserialize;
use sunrise::Coordinates;
use toml;
use crate::meta::NAME;

const CONFIG_DIR: &str = "/etc";
const CONFIG_FILENAME: &str = "config.toml";

/// Location settings
/// 
/// Currently this sets the location manually by latitude and longitude.
#[derive(Deserialize, Default)]
pub(crate) struct Location {
	/// Latitude
	pub lat: f64,

	/// Longitude
	pub lon: f64,

	/// Height AMSL
	pub alt: Option<f64>
}

impl From<Location> for Coordinates {
	fn from(value: Location) -> Self {
		Self::from(&value)
	}
}

impl From<&Location> for Coordinates {
	fn from(value: &Location) -> Self {
		Coordinates::new(value.lat, value.lon).unwrap_or(Coordinates::new(0.0, 0.0).unwrap())
	}
}

/// Brightness settings
/// 
/// This set the display brightness for day and night.
/// Values have to be set as [f64].
#[derive(Deserialize)]
pub(crate) struct Brightness {
	/// Day brightness percentage
	pub day: f64,

	/// Night brightness percentage
	pub night: f64
}

impl Default for Brightness {
	fn default() -> Self {
		Self { day: 1.0, night: 0.4 }
	}
}

/// Main config
#[derive(Deserialize, Default)]
pub(crate) struct Config {
	pub location: Location,
	pub brightness: Brightness
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