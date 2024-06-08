use crate::{version::Version, FMRI};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// [`Segment`] is a part of [`Version`] in [`FMRI`]
///
/// # Examples
///
/// ```plain
/// 1.2.3
/// 2023.0.0.5
/// 4
/// ```
///
/// it isn't
/// ```plain
/// .32.4
/// 3.a.4
/// ```
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Segment(Vec<i32>);

impl Segment {
    /// Returns [`Segment`] as [String] ("1.2.3")
    pub fn as_string(&self) -> String {
        self.get_ref()
            .iter()
            .map(|segment| segment.to_string())
            .collect::<Vec<String>>()
            .join(".")
    }

    /// Returns [`Segment`] as [Vec]<[i32]>
    pub fn get(self) -> Vec<i32> {
        self.0
    }

    /// Returns [`Segment`] as &[Vec]<[i32]>
    pub fn get_ref(&self) -> &Vec<i32> {
        &self.0
    }

    /// Returns [`Segment`] as &mut [Vec]<[i32]>
    pub fn get_ref_mut(&mut self) -> &mut Vec<i32> {
        &mut self.0
    }
}

impl TryFrom<String> for Segment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(&value as &str)
    }
}

impl TryFrom<&str> for Segment {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(
            value
                .split('.')
                .map(|s| {
                    s.parse::<i32>()
                        .map_err(|_| format!("invalid character in {}", s))
                })
                .collect::<Result<Vec<i32>, String>>()?,
        ))
    }
}

impl Display for Segment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}
