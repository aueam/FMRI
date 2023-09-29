pub mod segment;
pub mod segments;

use std::{
    cmp::Ordering,
    fmt::{Debug, Display, Formatter}
};
use serde::{Deserialize, Serialize};
use crate::{
    FMRI,
    compare::Compare,
    version::segments::Segments
};

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
#[derive(PartialEq, Serialize, Deserialize, Clone, Ord, Eq, PartialOrd)]
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
    pub fn new(mut version: String) -> Self {
        if !version.starts_with("@") {
            version.insert(0, '@')
        }

        Self {
            component_version: Segments::get_segment_from_string(version.clone(), '@'),
            build_version: Segments::get_segment_from_string(version.clone(), ','),
            branch_version: Segments::get_segment_from_string(version.clone(), '-'),
            timestamp: Segments::get_segment_from_string(version.clone(), ':'),
        }
    }

    /// Parses [`Version`] from raw [`FMRI`]
    ///
    /// # Examples
    ///
    /// ```
    /// use fmri::version::Version;
    /// let version = Version::parse_version_from_raw_fmri("fmri=pkg:/image/library/libpng16@1.6.34-2018.0.0.0".to_owned()).unwrap();
    /// assert_eq!(version, Version::new("@1.6.34-2018.0.0.0".to_owned()));
    ///
    /// let version = Version::parse_version_from_raw_fmri("fmri=pkg:/image/library/libpng16".to_owned());
    /// assert_eq!(version, None);
    /// ```
    ///
    pub fn parse_version_from_raw_fmri(raw_fmri: String) -> Option<Self> {
        // remove "fmri=" if present
        let mut raw_fmri = raw_fmri.trim_start_matches("fmri=").to_owned();

        // check if raw_fmri has version
        return match raw_fmri.find("@") {
            None => None,
            Some(position) => {
                let version = raw_fmri.split_off(position);

                Some(Self::new(version.to_owned()))
            }
        }
    }
}

impl Compare for Version {
    /// Compares versions, ignores a segment if one of them doesn't have it
    ///
    /// Ordering::Greater == self is newer<br>
    /// Ordering::Less == self is older<br>
    /// Ordering::Equal == versions are same<br>
    ///
    /// # examples
    ///
    /// ```
    /// use std::cmp::Ordering;
    /// use fmri::compare::Compare;
    /// use fmri::version::Version;
    ///
    /// let a = Version::new("2.1.1,5.11-2017.0.0.0:20171212T185746Z".to_owned());
    /// let b = Version::new("2.1.1,5.11-2017.0.0.0:20171212T185746Z".to_owned());
    /// assert_eq!(a.compare(&b), Ordering::Equal);
    ///
    /// let a = Version::new("2.1.1,5.11-2018.0.0.0:20171212T185746Z".to_owned());
    /// let b = &Version::new("2.1.1,5.11-2017.0.0.0:20171212T185746Z".to_owned());
    /// assert_eq!(a.compare(&b), Ordering::Greater);
    ///
    /// let a = Version::new("2.1.1,5.11-2017.0.0.0:20171212T185746Z".to_owned());
    /// let b = &Version::new("2.1.1,5.11-2018.0.0.0:20171212T185746Z".to_owned());
    /// assert_eq!(a.compare(&b), Ordering::Less);
    /// ```
    fn compare(&self, comparing_to: &Self) -> Ordering {
        if let Segments::ComponentVersion(self_segment) = &self.component_version {
            if let Segments::ComponentVersion(comparing_to_segment) = &comparing_to.component_version {
                match self_segment.compare(&comparing_to_segment) {
                    Ordering::Greater => { return Ordering::Greater; }
                    Ordering::Less => { return Ordering::Less; }
                    Ordering::Equal => { }
                }
            }
        }

        if let Segments::BuildVersion(self_segment) = &self.build_version {
            if let Segments::BuildVersion(comparing_to_segment) = &comparing_to.build_version {
                match self_segment.compare(&comparing_to_segment) {
                    Ordering::Greater => { return Ordering::Greater; }
                    Ordering::Less => { return Ordering::Less; }
                    Ordering::Equal => { }
                }
            }
        }

        if let Segments::BranchVersion(self_segment) = &self.branch_version {
            if let Segments::BranchVersion(comparing_to_segment) = &comparing_to.branch_version {
                match self_segment.compare(&comparing_to_segment) {
                    Ordering::Greater => { return Ordering::Greater; }
                    Ordering::Less => { return Ordering::Less; }
                    Ordering::Equal => { }
                }
            }
        }

        // ignoring timestamp cuz timestamp does not determine if the package is newer

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