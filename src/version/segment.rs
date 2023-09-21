use std::{
    cmp::Ordering,
    fmt::{Display, Formatter},
};
use serde::{Deserialize, Serialize};
use crate::{
    FMRI,
    compare::Compare,
    version::Version
};

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
    /// Parse "1.2.3" into [`Segment`]
    pub fn from_string(segment: String) -> Self {
        let str_vec: Vec<&str> = segment.split(".").collect();

        Self(
            str_vec
                .iter()
                .map(|&s| s.parse::<i32>().expect(&format!("invalid character in {}", s)))
                .collect()
        )
    }

    /// Returns [`Segment`] as [`String`] ("1.2.3")
    pub fn as_string(&self) -> String {
        let mut string: &mut String = &mut "".to_owned();

        for (index, segment) in self.get_ref().iter().enumerate() {
            string.push_str(&segment.to_string() as &str);

            if index + 1 != self.get_ref().len() {
                string.push_str(".");
            }
        }

        string.clone()
    }

    /// Returns [`Segment`] as [Vec]<[i32]>
    pub fn get(self) -> Vec<i32> {
        self.0
    }

    /// Returns [`Segment`] as &[Vec]<[`i32`]>
    pub fn get_ref(&self) -> &Vec<i32> {
        &self.0
    }

    /// Returns [`Segment`] as &mut [Vec]<[`i32`]>
    pub fn get_ref_mut(&mut self) -> &mut Vec<i32> {
        &mut self.0
    }
}

/// Implementation of [`Compare`] for [`Segment`]
impl Compare for Segment {
    /// Compares version segments
    ///
    /// Ordering::Greater == self is newer<br>
    /// Ordering::Less == self is older<br>
    /// Ordering::Equal == segments are same<br>
    ///
    /// # examples
    ///
    /// ```
    /// use std::cmp::Ordering;
    /// use fmri::compare::Compare;
    /// use fmri::version::segment::Segment;
    /// let a = Segment::from_string("1.0".to_owned());
    /// let b = Segment::from_string("1.0".to_owned());
    /// assert_eq!(a.compare(&b), Ordering::Equal);
    ///
    /// let a = Segment::from_string("1.1".to_owned());
    /// let b = Segment::from_string("1.0".to_owned());
    /// assert_eq!(a.compare(&b), Ordering::Greater);
    ///
    /// let a = Segment::from_string("1.0".to_owned());
    /// let b = Segment::from_string("1.1".to_owned());
    /// assert_eq!(a.compare(&b), Ordering::Less);
    /// ```
    fn compare(&self, comparing_to: &Self) -> Ordering {
        return match self.get_ref().len().cmp(&comparing_to.get_ref().len()) {
            Ordering::Greater => {
                for (index, num) in comparing_to.get_ref().iter().enumerate() {
                    if num > &self.get_ref()[index] {
                        return Ordering::Less;
                    } else if num < &self.get_ref()[index] {
                        return Ordering::Greater;
                    }
                }
                Ordering::Greater
            }
            Ordering::Less => {
                for (index, num) in self.get_ref().iter().enumerate() {
                    if num > &comparing_to.get_ref()[index] {
                        return Ordering::Greater;
                    } else if num < &comparing_to.get_ref()[index] {
                        return Ordering::Less;
                    }
                }
                Ordering::Less
            }
            Ordering::Equal => {
                for (index, num) in self.get_ref().iter().enumerate() {
                    if num > &comparing_to.get_ref()[index] {
                        return Ordering::Greater;
                    } else if num < &comparing_to.get_ref()[index] {
                        return Ordering::Less;
                    }
                }
                Ordering::Equal
            }
        };
    }
}

/// Implementation of [`Display`] for [`Segment`]
impl Display for Segment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}