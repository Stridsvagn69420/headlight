/// sysfs backlight-class root directory
const SYSFS_BACKLIGHT_ROOT: &str = "/sys/class/backlight";

/// Backlight
///
/// Struct representing an entry in `/sys/class/backlight`.
pub struct Backlight {
    /// ID
    pub id: String
}