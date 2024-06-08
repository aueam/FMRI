use std::fmt::{Debug, Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::{
    helpers::{check_character_collision, remove_first_and_last_characters},
    FMRI,
};

/// [`Publisher`] is a part of [`FMRI`]
///
/// # Examples
///
/// there is publisher
/// ```plain
/// pkg://publisher/*
/// ```
///
/// there isn't publisher
/// ```plain
/// pkg:/*
/// ```
///
/// `* = continues package name`
#[derive(PartialEq, Serialize, Deserialize, Clone, Ord, Eq, PartialOrd)]
pub struct Publisher(String);

impl Publisher {
    pub fn new(mut publisher: String) -> Result<Self, String> {
        check_character_collision(&publisher)?;
        publisher = remove_first_and_last_characters(&publisher, '/').to_owned();
        Ok(Self(publisher))
    }

    /// Parses [`Publisher`] from raw [`FMRI`], returns [`Publisher`] and [String] after [`Publisher`]
    ///
    /// # Examples
    ///
    /// there is publisher
    /// ```
    /// use fmri::publisher::Publisher;
    /// let publisher = Publisher::parse_publisher_from_raw_fmri("fmri=pkg://publisher/test/test@1-0.1".to_owned()).unwrap().unwrap();
    /// assert_eq!(publisher, Publisher::new("publisher".to_owned()).unwrap());
    /// ```
    ///
    /// there isn't publisher
    /// ```
    /// use fmri::publisher::Publisher;
    /// let publisher = Publisher::parse_publisher_from_raw_fmri("pkg:/test/test@1-0.1".to_owned()).unwrap();
    /// assert_eq!(publisher, None);
    /// ```
    ///
    pub fn parse_publisher_from_raw_fmri(raw_fmri: String) -> Result<Option<Self>, String> {
        // remove "fmri=" if present
        let raw_fmri = raw_fmri.trim_start_matches("fmri=").to_owned();

        // check if raw_fmri has publisher
        return match raw_fmri.find("pkg://") {
            None => Ok(None),
            Some(position) => {
                if position != 0 {
                    panic!(
                        "wrong position of starting \"pkg://\" pattern ({})",
                        position
                    )
                }

                let (publisher, _) = raw_fmri
                    .trim_start_matches("pkg://")
                    .split_once('/')
                    .expect("Fmri must contain \"/package_name\"");
                Ok(Some(Self::new(publisher.to_owned())?))
            }
        };
    }

    pub fn get_as_string(self) -> String {
        self.0
    }

    pub fn get_as_ref_string(&self) -> &String {
        &self.0
    }

    pub fn get_as_ref_mut_string(&mut self) -> &mut String {
        &mut self.0
    }
}

impl Display for Publisher {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "pkg://{}/", self.get_as_ref_string())
    }
}

impl Debug for Publisher {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
