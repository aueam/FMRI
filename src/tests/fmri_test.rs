use std::cmp::Ordering;
use crate::FMRI;
use crate::compare::Compare;

#[test]
fn compare() {
    let a = FMRI::parse_raw(&"test@1,1-1:20220913T082027Z".to_owned());
    let b = FMRI::parse_raw(&"test@1-1:20220913T082027Z".to_owned());

    assert_eq!(a.compare(&b), Ordering::Equal);

    let a = FMRI::parse_raw(&"test".to_owned());
    let b = FMRI::parse_raw(&"test".to_owned());

    assert_eq!(a.compare(&b), Ordering::Equal);

    let a = FMRI::parse_raw(&"test@1".to_owned());
    let b = FMRI::parse_raw(&"test".to_owned());

    assert_eq!(a.compare(&b), Ordering::Equal);

    let a = FMRI::parse_raw(&"test@2".to_owned());
    let b = FMRI::parse_raw(&"test@1".to_owned());

    assert_eq!(a.compare(&b), Ordering::Greater);

    let a = FMRI::parse_raw(&"test@1".to_owned());
    let b = FMRI::parse_raw(&"test@2".to_owned());

    assert_eq!(a.compare(&b), Ordering::Less);
}