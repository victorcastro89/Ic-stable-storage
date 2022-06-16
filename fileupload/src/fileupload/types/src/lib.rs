mod timestamped;
use std::fmt;

pub use timestamped::*;
pub type TimestampMillis = u64;
pub type TimestampNanos = u64;
pub enum SupportedFormats {
    JPEG,
    JPG,
    PNG,
    GIF,
}

impl fmt::Display for SupportedFormats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SupportedFormats::JPEG => write!(f, "jpeg"),
            SupportedFormats::JPG => write!(f, "jpg"),
            SupportedFormats::PNG => write!(f, "png"),
            SupportedFormats::GIF => write!(f, "gif"),
        }
    }
}
