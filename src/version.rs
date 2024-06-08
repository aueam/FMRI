use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::{version::segments::Segments, FMRI};

pub mod segment;
pub mod segments;

/// [`Version`] is a part of [`FMRI`]
///
/// # Examples
///
/// ```plain
/// *@2.1.1,5.11-2017.0.0.0:20171212T185746Z
/// *@2.1.1-2017.0.0.0
/// *@2.1
/// *@1
/// *@2-2
/// ```
///
/// `* = continues package name`
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Version {
    component_version: Segments,
    /// Build_version is optional
    build_version: Segments,
    /// Branch_version is optional
    branch_version: Segments,
    /// Timestamp is optional
    timestamp: Segments,
}

impl Version {
    /// Parses "@2.1.1,5.11-2017.0.0.0:20171212T185746Z" into [`Version`]
    ///
    /// # Error
    ///
    /// Returns a string with error message if one of the segments is invalid
    pub fn new(mut version: String) -> Result<Self, String> {
        if !version.starts_with('@') {
            version.insert(0, '@')
        }

        Ok(Self {
            component_version: Segments::get_segment_from_string(version.clone(), '@')?,
            build_version: Segments::get_segment_from_string(version.clone(), ',')?,
            branch_version: Segments::get_segment_from_string(version.clone(), '-')?,
            timestamp: Segments::get_segment_from_string(version.clone(), ':')?,
        })
    }

    /// Parses [`Version`] from raw [`FMRI`]
    ///
    /// # Examples
    ///
    /// ```
    /// use fmri::version::Version;
    /// let version = Version::parse_version_from_raw_fmri("fmri=pkg:/image/library/libpng16@1.6.34-2018.0.0.0".to_owned()).unwrap().unwrap();
    /// assert_eq!(version, Version::new("@1.6.34-2018.0.0.0".to_owned()).unwrap());
    ///
    /// let version = Version::parse_version_from_raw_fmri("fmri=pkg:/image/library/libpng16".to_owned()).unwrap();
    /// assert_eq!(version, None);
    /// ```
    ///
    /// # Error
    ///
    /// Returns a string with error message if one of the segments is invalid
    pub fn parse_version_from_raw_fmri(raw_fmri: String) -> Result<Option<Self>, String> {
        // remove "fmri=" if present
        let mut raw_fmri = raw_fmri.trim_start_matches("fmri=").to_owned();

        // check if raw_fmri has version
        match raw_fmri.find('@') {
            None => Ok(None),
            Some(position) => {
                let version = raw_fmri.split_off(position);

                Ok(Some(Self::new(version.to_owned())?))
            }
        }
    }
}

impl PartialOrd<Self> for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        if let (
            Segments::ComponentVersion(self_segment),
            Segments::ComponentVersion(other_segment),
        ) = (&self.component_version, &other.component_version)
        {
            if self_segment != other_segment {
                return self_segment.cmp(other_segment);
            }
        }

        if let (Segments::BuildVersion(self_segment), Segments::BuildVersion(other_segment)) =
            (&self.build_version, &other.build_version)
        {
            if self_segment != other_segment {
                return self_segment.cmp(other_segment);
            }
        }

        if let (Segments::BranchVersion(self_segment), Segments::BranchVersion(other_segment)) =
            (&self.branch_version, &other.branch_version)
        {
            if self_segment != other_segment {
                return self_segment.cmp(other_segment);
            }
        }

        // ignoring timestamp because timestamp does not determine if the package is newer

        Ordering::Equal
    }
}

/// Implementation of [`Display`] for [`Version`]
impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string: &mut String = &mut "".to_string();

        if let Segments::ComponentVersion(segment) = &self.component_version {
            string.push('@');
            string.push_str(&segment.as_string());
        }

        if let Segments::BuildVersion(segment) = &self.build_version {
            string.push(',');
            string.push_str(&segment.as_string());
        }

        if let Segments::BranchVersion(segment) = &self.branch_version {
            string.push('-');
            string.push_str(&segment.as_string());
        }

        if let Segments::Timestamp(segment) = &self.timestamp {
            string.push(':');
            string.push_str(segment);
        }

        write!(f, "{}", string)
    }
}

/// Implementation of [`Debug`] for [`Version`]
impl Debug for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
