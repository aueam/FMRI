use std::cmp::Ordering;

/// Implementation of [`Compare`] trait
pub trait Compare {
    /// Compares structures
    ///
    /// expected output:<br>
    /// Ordering::Greater == self is newer<br>
    /// Ordering::Less == self is older<br>
    /// Ordering::Equal == versions are same<br>
    fn compare(&self, comparing_to: &Self) -> Ordering;
}