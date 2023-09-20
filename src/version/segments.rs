use std::fmt::{Debug, Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::version::segment::Segment;

/// [`Segments`] is a part of [`Version`] in [`FMRI`]
///
/// # Examples
///
/// ```plain
/// @1.2.3
/// -2023.0.0.5
/// ,5.11
/// :20171212T185746Z
/// ```
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Segments {
    /// after '@'
    ComponentVersion(Segment),
    /// after ','
    BuildVersion(Segment),
    /// after '-'
    BranchVersion(Segment),
    /// after ':'
    Timestamp(String), // TODO: implement timestamp struct?
    /// Some segments are not needed
    None,
}

impl Segments {
    /// Separates [`Segment`] from from inputted [`String`]
    ///
    /// # Example
    ///
    /// ```
    /// use fmri::version::segments::Segments;
    /// assert_eq!(
    ///     Segments::get_segment_from_string("@2.1.1-2018.0.0.0".to_string(), ','),
    ///     Segments::None
    /// );
    /// ```
    pub fn get_segment_from_string(mut string: String, segment_starts_with: char) -> Self {
        let mut end = match string.find(segment_starts_with) {
            None => return Self::None,
            Some(position) => {
                string.split_off(position + 1)
            }
        };

        for (index, c) in end.clone().chars().enumerate() {
            match c {
                ',' | '-' | ':' => {
                    let _ = end.split_off(index);
                    break;
                }
                '0'..='9' | '.' => {}
                _ => {
                    break;
                }
            }
        }

        return match segment_starts_with {
            '@' => Self::ComponentVersion(Segment::from_string(end.clone())),
            ',' => Self::BuildVersion(Segment::from_string(end.clone())),
            '-' => Self::BranchVersion(Segment::from_string(end.clone())),
            ':' => Self::Timestamp(end.clone()),
            _ => Self::None
        }
    }
}

/// Implementation of [`Display`] for [`Segments`]
impl Display for Segments {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_owned())
    }
}