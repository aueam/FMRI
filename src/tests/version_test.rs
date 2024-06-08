use crate::version::segment::Segment;
use crate::version::segments::Segments;
use crate::version::Version;

#[test]
fn version_compare() {
    assert_eq!(
        Version::new("2.1.1,5.11-2017.0.0.0:20171212T185746Z".to_owned()),
        Version::new("2.1.1,5.11-2017.0.0.0:20171212T185746Z".to_owned())
    );
    assert_eq!(Version::new("1".to_owned()), Version::new("1".to_owned()));
    assert!(
        Version::new("2.1.1,5.11-2018.0.0.0:20171212T185746Z".to_owned())
            > Version::new("2.1.1,5.11-2017.0.0.0:20171212T185746Z".to_owned())
    );
    assert!(
        Version::new("2.1.1,5.11-2017.0.0.0".to_owned())
            < Version::new("2.1.1,5.11-2018.0.0.0:20171212T185746Z".to_owned())
    );
    assert!(
        Version::new("2.1.1-2017.0.0.0".to_owned()) < Version::new("2.1.1-2018.0.0.0".to_owned())
    );
    assert!(
        Version::new("2.1.1-2018.0.0.0".to_owned()) > Version::new("2.1.1-2017.0.0.0".to_owned())
    );
    assert!(
        Version::new("2.1.1-2018.0.0.0".to_owned())
            > Version::new("2.1.1,5.11-2017.0.0.0".to_owned())
    );

    assert!(
        Version::new("2.1.1-2017.0.0.0".to_owned())
            < Version::new("2.1.1,5.11-2018.0.0.0".to_owned())
    );
}

#[test]
fn segment_compare() {
    assert_eq!(
        Segment::try_from("1.0").unwrap(),
        Segment::try_from("1.0").unwrap()
    );
    assert!(Segment::try_from("1.1").unwrap() > Segment::try_from("1.0").unwrap());
    assert!(Segment::try_from("1.0").unwrap() < Segment::try_from("1.1").unwrap());
    assert!(Segment::try_from("1.1").unwrap() > Segment::try_from("1.0.1").unwrap());
    assert!(Segment::try_from("1.1.1").unwrap() > Segment::try_from("1.1").unwrap());
}

#[test]
fn get_segment_from_string() {
    assert_eq!(
        Segments::get_segment_from_string(
            "@2.1.1,5.11-2018.0.0.0:20171212T185746Z".to_string(),
            '@'
        )
        .unwrap(),
        Segments::ComponentVersion(Segment::try_from("2.1.1").unwrap())
    );
    assert_eq!(
        Segments::get_segment_from_string(
            "@2.1.1,5.11-2018.0.0.0:20171212T185746Z".to_string(),
            ','
        )
        .unwrap(),
        Segments::BuildVersion(Segment::try_from("5.11").unwrap())
    );
    assert_eq!(
        Segments::get_segment_from_string(
            "@2.1.1,5.11-2018.0.0.0:20171212T185746Z".to_string(),
            '-'
        )
        .unwrap(),
        Segments::BranchVersion(Segment::try_from("2018.0.0.0").unwrap())
    );
    assert_eq!(
        Segments::get_segment_from_string(
            "@2.1.1,5.11-2018.0.0.0:20171212T185746Z".to_string(),
            ':'
        )
        .unwrap(),
        Segments::Timestamp("20171212T185746Z".to_owned())
    );

    assert_eq!(
        Segments::get_segment_from_string("@2.1.1-2018.0.0.0".to_string(), '@').unwrap(),
        Segments::ComponentVersion(Segment::try_from("2.1.1").unwrap())
    );
    assert_eq!(
        Segments::get_segment_from_string("@2.1.1-2018.0.0.0".to_string(), ',').unwrap(),
        Segments::None
    );
    assert_eq!(
        Segments::get_segment_from_string("@2.1.1-2018.0.0.0".to_string(), '-').unwrap(),
        Segments::BranchVersion(Segment::try_from("2018.0.0.0").unwrap())
    );
    assert_eq!(
        Segments::get_segment_from_string("@2.1.1-2018.0.0.0".to_string(), ':').unwrap(),
        Segments::None
    );
}

#[test]
#[should_panic]
fn get_segment_from_string_panic_1() {
    assert_eq!(
        Segments::get_segment_from_string(
            "@2.1a.1,5.c11-2018.g0.0.0:2017121b2T185746Z".to_string(),
            '@'
        )
        .unwrap(),
        Segments::ComponentVersion(Segment::try_from("2.1.1").unwrap())
    );
}

#[test]
#[should_panic]
fn get_segment_from_string_panic_2() {
    assert_eq!(
        Segments::get_segment_from_string(
            "@2.1a.1,5.c11-2018.g0.0.0:2017121b2T185746Z".to_string(),
            ','
        )
        .unwrap(),
        Segments::BuildVersion(Segment::try_from("5.11").unwrap())
    );
}

#[test]
#[should_panic]
fn get_segment_from_string_panic_3() {
    assert_eq!(
        Segments::get_segment_from_string(
            "@2.1a.1,5.c11-2018.g0.0.0:2017121b2T185746Z".to_string(),
            '-'
        )
        .unwrap(),
        Segments::BranchVersion(Segment::try_from("2018.0.0.0").unwrap())
    );
}

#[test]
#[should_panic]
fn get_segment_from_string_panic_4() {
    assert_eq!(
        Segments::get_segment_from_string(
            "@2.1a.1,5.c11-2018.g0.0.0:2017121b2T185746Z".to_string(),
            ':'
        )
        .unwrap(),
        Segments::Timestamp("20171212T185746Z".to_owned())
    );
}

#[test]
fn parse_version_from_raw_fmri() {
    let version = Version::parse_version_from_raw_fmri(
        "fmri=pkg:/image/library/libpng16@1.6.34-2018.0.0.0".to_owned(),
    )
    .unwrap()
    .unwrap();
    assert_eq!(
        version,
        Version::new("@1.6.34-2018.0.0.0".to_owned()).unwrap()
    );

    let version =
        Version::parse_version_from_raw_fmri("fmri=pkg:/image/library/libpng16".to_owned())
            .unwrap();
    assert_eq!(version, None);
}
