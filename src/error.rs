use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WebPSimpleError;

impl fmt::Display for WebPSimpleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("WebP format error")
    }
}

impl std::error::Error for WebPSimpleError {}
