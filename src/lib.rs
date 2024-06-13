use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use std::fmt::{Debug, Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::helpers::{check_character_collision, remove_first_and_last_characters};

pub use self::{fmri_list::FMRIList, publisher::Publisher, version::Version};

pub mod fmri_list;
mod helpers;
pub mod publisher;
#[cfg(test)]
mod tests;
pub mod version;

/// [`FMRI`] represents pkg fmri versioning system
///
/// # Examples
///
/// ```plain
/// pkg:/audio/audacity
/// pkg:/audio/audacity@2.3.2,5.11-2022.0.0.1
/// pkg://solaris/system/library
/// pkg://solaris/system/library@0.5.11-0.175.1.0.0.2.1:20120919T082311Z
/// ```
#[derive(PartialEq, Serialize, Deserialize, Clone, Eq, Hash)]
pub struct FMRI {
    /// Publisher is optional
    publisher: Option<Publisher>,
    // TODO: add package_name struct?
    package_name: String,
    /// Version is optional
    version: Option<Version>,
}

impl FMRI {
    /// Returns [`FMRI`] with given package name
    pub fn new_from_package_name(mut package_name: String) -> Result<Self, String> {
        if package_name.is_empty() {
            panic!("package name can't be empty")
        }

        check_character_collision(&package_name)?;
        package_name = remove_first_and_last_characters(&package_name, '/').to_owned();

        Ok(Self {
            publisher: None,
            package_name,
            version: None,
        })
    }

    /// Returns [`FMRI`] from raw fmri
    ///
    /// # Examples
    ///
    /// ```
    /// use fmri::FMRI;
    /// FMRI::parse_raw(&"fmri=test@1-1:20220913T082027Z".to_owned()).unwrap();
    /// FMRI::parse_raw(&"pkg://publisher/test@1-1:20220913T082027Z".to_owned()).unwrap();
    /// ```
    ///
    /// # Error
    ///
    /// Returns a string with error message if one of the segments is invalid
    pub fn parse_raw(raw_fmri: &str) -> Result<Self, String> {
        let mut publisher: Option<Publisher> = None;
        let mut version: Option<Version> = None;
        let mut package_name: String = raw_fmri.to_owned().trim_start_matches("fmri=").to_owned();

        match Publisher::parse_publisher_from_raw_fmri(raw_fmri.to_owned()) {
            Ok(None) => {
                package_name = package_name.trim_start_matches("pkg:/").to_owned();
            }
            Ok(Some(p)) => {
                publisher = Some(p);
                let (_, end_str) = package_name
                    .trim_start_matches("pkg://")
                    .split_once('/')
                    .expect("Fmri must contain \"/package_name\"");
                package_name = end_str.to_owned()
            }
            Err(e) => return Err(e),
        }

        match Version::parse_version_from_raw_fmri(raw_fmri.to_owned()) {
            Ok(None) => {}
            Ok(Some(v)) => {
                version = Some(v);
                let (start_str, _) = package_name.split_once('@').expect("error");
                package_name = start_str.to_owned()
            }
            Err(e) => return Err(e),
        }

        let mut fmri = Self::new_from_package_name(package_name)?;
        if let Some(p) = publisher {
            fmri.change_publisher(p);
        }
        if let Some(v) = version {
            fmri.change_version(v);
        }
        Ok(fmri)
    }

    /// Checks if package names are same
    pub fn package_name_eq(&self, comparing_to: &FMRI) -> bool {
        self.get_package_name_as_ref_string()
            .eq(comparing_to.get_package_name_as_ref_string())
    }

    pub fn get_package_name_as_string(self) -> String {
        self.package_name
    }

    pub fn get_package_name_as_ref_string(&self) -> &String {
        &self.package_name
    }

    pub fn get_package_name_as_ref_mut_string(&mut self) -> &mut String {
        &mut self.package_name
    }

    pub fn get_publisher(self) -> Option<Publisher> {
        self.publisher
    }

    pub fn get_publisher_ref(&self) -> &Option<Publisher> {
        &self.publisher
    }

    pub fn get_publisher_ref_mut(&mut self) -> &mut Option<Publisher> {
        &mut self.publisher
    }

    pub fn has_publisher(&self) -> bool {
        self.publisher.is_some()
    }

    pub fn change_publisher(&mut self, publisher: Publisher) {
        self.publisher = Some(publisher);
    }

    /// Returns [`None`] if there isn't [`Publisher`]
    pub fn get_publisher_as_ref_string(&self) -> Option<&String> {
        if let Some(publisher) = &self.publisher {
            return Some(publisher.get_as_ref_string());
        }
        None
    }

    pub fn remove_publisher(&mut self) {
        self.publisher = None
    }

    pub fn get_version(self) -> Option<Version> {
        self.version
    }

    pub fn get_version_ref(&self) -> &Option<Version> {
        &self.version
    }
    pub fn get_version_ref_mut(&mut self) -> &mut Option<Version> {
        &mut self.version
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    pub fn change_version(&mut self, version: Version) {
        self.version = Some(version)
    }

    /// Returns [`None`] if there isn't [`Version`]
    pub fn get_version_as_string(&self) -> Option<String> {
        if let Some(version) = &self.version {
            return Some(format!("{}", version));
        }
        None
    }

    pub fn remove_version(&mut self) -> &mut FMRI {
        self.version = None;
        self
    }
}

impl PartialOrd<Self> for FMRI {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FMRI {
    /// Compares versions of FMRI
    fn cmp(&self, other: &Self) -> Ordering {
        self.version
            .as_ref()
            .and_then(|ver| other.version.as_ref().map(|ver2| ver.cmp(ver2)))
            .unwrap_or(Equal)
    }
}

impl Display for FMRI {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut string: String = "".to_owned();

        if let Some(publisher) = self.get_publisher_as_ref_string() {
            string.push_str("pkg://");
            string.push_str(publisher);
            string.push('/');
        } else {
            string.push_str("pkg:/");
        }

        string.push_str(self.get_package_name_as_ref_string());

        if let Some(version) = self.get_version_as_string() {
            string.push_str(&version)
        }

        write!(f, "{}", string)
    }
}

impl Debug for FMRI {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
