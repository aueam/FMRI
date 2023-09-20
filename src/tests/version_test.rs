use std::cmp::Ordering;
use crate::compare::Compare;
use crate::version::segment::Segment;
use crate::version::segments::Segments;
use crate::version::Version;

#[test]
fn version_compare() {
    let a = Version::new("2.1.1,5.11-2017.0.0.0:20171212T185746Z".to_owned());
    let b = Version::new("2.1.1,5.11-2017.0.0.0:20171212T185746Z".to_owned());
    assert_eq!(a.compare(&b), Ordering::Equal);

    let a = Version::new("1".to_owned());
    let b = Version::new("1".to_owned());
    assert_eq!(a.compare(&b), Ordering::Equal);

    let a = Version::new("2.1.1,5.11-2018.0.0.0:20171212T185746Z".to_owned());
    let b = Version::new("2.1.1,5.11-2017.0.0.0:20171212T185746Z".to_owned());
    assert_eq!(a.compare(&b), Ordering::Greater);

    let a = Version::new("2.1.1,5.11-2017.0.0.0:20171212T185746Z".to_owned());
    let b = Version::new("2.1.1,5.11-2018.0.0.0:20171212T185746Z".to_owned());
    assert_eq!(a.compare(&b), Ordering::Less);

    let a = Version::new("2.1.1,5.11-2017.0.0.0".to_owned());
    let b = Version::new("2.1.1,5.11-2018.0.0.0:20171212T185746Z".to_owned());
    assert_eq!(a.compare(&b), Ordering::Less);

    let a = Version::new("2.1.1-2017.0.0.0".to_owned());
    let b = Version::new("2.1.1-2018.0.0.0".to_owned());
    assert_eq!(a.compare(&b), Ordering::Less);

    let a = Version::new("2.1.1-2018.0.0.0".to_owned());
    let b = Version::new("2.1.1-2017.0.0.0".to_owned());
    assert_eq!(a.compare(&b), Ordering::Greater);

    let a = Version::new("2.1.1-2018.0.0.0".to_owned());
    let b = Version::new("2.1.1,5.11-2017.0.0.0".to_owned());
    assert_eq!(a.compare(&b), Ordering::Greater);

    let a = Version::new("2.1.1-2017.0.0.0".to_owned());
    let b = Version::new("2.1.1,5.11-2018.0.0.0".to_owned());
    assert_eq!(a.compare(&b), Ordering::Less);
}

#[test]
fn segment_compare() {
    let a = Segment::from_string("1.0".to_owned());
    let b = Segment::from_string("1.0".to_owned());
    assert_eq!(a.compare(&b), Ordering::Equal);

    let a = Segment::from_string("1.1".to_owned());
    let b = Segment::from_string("1.0".to_owned());
    assert_eq!(a.compare(&b), Ordering::Greater);

    let a = Segment::from_string("1.0".to_owned());
    let b = Segment::from_string("1.1".to_owned());
    assert_eq!(a.compare(&b), Ordering::Less);

    let a = Segment::from_string("1.0.1".to_owned());
    let b = Segment::from_string("1.1".to_owned());
    assert_eq!(a.compare(&b), Ordering::Less);

    let a = Segment::from_string("1.1.1".to_owned());
    let b = Segment::from_string("1.1".to_owned());
    assert_eq!(a.compare(&b), Ordering::Greater);

    let a = Segment::from_string("1.1".to_owned());
    let b = Segment::from_string("1.0.1".to_owned());
    assert_eq!(a.compare(&b), Ordering::Greater);

    let a = Segment::from_string("1.1".to_owned());
    let b = Segment::from_string("1.1.1".to_owned());
    assert_eq!(a.compare(&b), Ordering::Less);
}

#[test]
fn get_segment_from_string() {
    assert_eq!(
        Segments::get_segment_from_string("@2.1.1,5.11-2018.0.0.0:20171212T185746Z".to_string(), '@'),
        Segments::ComponentVersion(Segment::from_string("2.1.1".to_owned()))
    );
    assert_eq!(
        Segments::get_segment_from_string("@2.1.1,5.11-2018.0.0.0:20171212T185746Z".to_string(), ','),
        Segments::BuildVersion(Segment::from_string("5.11".to_owned()))
    );
    assert_eq!(
        Segments::get_segment_from_string("@2.1.1,5.11-2018.0.0.0:20171212T185746Z".to_string(), '-'),
        Segments::BranchVersion(Segment::from_string("2018.0.0.0".to_owned()))
    );
    assert_eq!(
        Segments::get_segment_from_string("@2.1.1,5.11-2018.0.0.0:20171212T185746Z".to_string(), ':'),
        Segments::Timestamp("20171212T185746Z".to_owned())
    );

    assert_eq!(
        Segments::get_segment_from_string("@2.1.1-2018.0.0.0".to_string(), '@'),
        Segments::ComponentVersion(Segment::from_string("2.1.1".to_owned()))
    );
    assert_eq!(
        Segments::get_segment_from_string("@2.1.1-2018.0.0.0".to_string(), ','),
        Segments::None
    );
    assert_eq!(
        Segments::get_segment_from_string("@2.1.1-2018.0.0.0".to_string(), '-'),
        Segments::BranchVersion(Segment::from_string("2018.0.0.0".to_owned()))
    );
    assert_eq!(
        Segments::get_segment_from_string("@2.1.1-2018.0.0.0".to_string(), ':'),
        Segments::None
    );
}

#[test]
#[should_panic]
fn get_segment_from_string_panic_1() {
    assert_eq!(
        Segments::get_segment_from_string("@2.1a.1,5.c11-2018.g0.0.0:2017121b2T185746Z".to_string(), '@'),
        Segments::ComponentVersion(Segment::from_string("2.1.1".to_owned()))
    );
}

#[test]
#[should_panic]
fn get_segment_from_string_panic_2() {
    assert_eq!(
        Segments::get_segment_from_string("@2.1a.1,5.c11-2018.g0.0.0:2017121b2T185746Z".to_string(), ','),
        Segments::BuildVersion(Segment::from_string("5.11".to_owned()))
    );
}

#[test]
#[should_panic]
fn get_segment_from_string_panic_3() {
    assert_eq!(
        Segments::get_segment_from_string("@2.1a.1,5.c11-2018.g0.0.0:2017121b2T185746Z".to_string(), '-'),
        Segments::BranchVersion(Segment::from_string("2018.0.0.0".to_owned()))
    );
}

#[test]
#[should_panic]
fn get_segment_from_string_panic_4() {
    assert_eq!(
        Segments::get_segment_from_string("@2.1a.1,5.c11-2018.g0.0.0:2017121b2T185746Z".to_string(), ':'),
        Segments::Timestamp("20171212T185746Z".to_owned())
    );
}

#[test]
fn parse_version_from_raw_fmri() {
    let version = Version::parse_version_from_raw_fmri("fmri=pkg:/image/library/libpng16@1.6.34-2018.0.0.0".to_owned()).unwrap();
    assert_eq!(version, Version::new("@1.6.34-2018.0.0.0".to_owned()));

    let version = Version::parse_version_from_raw_fmri("fmri=pkg:/image/library/libpng16".to_owned());
    assert_eq!(version, None);
}