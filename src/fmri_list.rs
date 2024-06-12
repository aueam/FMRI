use std::fmt::{Debug, Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::FMRI;

/// [`FMRIList`] contains more [`FMRIs`][FMRI]
#[derive(Serialize, Deserialize, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct FMRIList(Vec<FMRI>);

impl FMRIList {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn add(&mut self, fmri: FMRI) {
        self.0.push(fmri)
    }

    pub fn get(self) -> Vec<FMRI> {
        self.0
    }

    pub fn get_ref(&self) -> &Vec<FMRI> {
        &self.0
    }

    pub fn get_ref_mut(&mut self) -> &mut Vec<FMRI> {
        &mut self.0
    }

    pub fn len(&self) -> usize {
        self.get_ref().len()
    }

    pub fn is_empty(&self) -> bool {
        if self.get_ref().len() == 0 {
            return true;
        }
        false
    }

    pub fn contains(&self, checking_fmri: &FMRI) -> bool {
        for fmri in self.get_ref() {
            if fmri.package_name_eq(checking_fmri) {
                return true;
            }
        }
        false
    }
}

impl From<Vec<FMRI>> for FMRIList {
    fn from(value: Vec<FMRI>) -> Self {
        Self(value)
    }
}

impl Display for FMRIList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut string: String = "".to_owned();

        let fmris = self.get_ref();
        let len = fmris.len() - 1;
        for (index, fmri) in fmris.iter().enumerate() {
            string.push_str(&format!("{}", fmri));

            if index < len {
                string.push_str(", ");
            }
        }

        write!(f, "{}", string)
    }
}

impl Debug for FMRIList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut string: String = "".to_owned();

        for (index, fmri) in self.get_ref().iter().enumerate() {
            string.push_str(&format!("{}. {}\n", index + 1, fmri));
        }

        write!(f, "{}", string)
    }
}

impl Default for FMRIList {
    fn default() -> Self {
        Self::new()
    }
}
