//!
//!
//!

use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;

pub enum TGAError {
    Header = 1,
}

impl Error for TGAError {}

impl Debug for TGAError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Header => "文件头解释失败",
            }
        )
    }
}

impl Display for TGAError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Header => "文件头解释失败",
            }
        )
    }
}
