use std::fmt::Display;
use serde::Deserialize;

/// Main config
#[derive(Deserialize)]
struct Config {
	location: Location,
	brightness: Brightness
}

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