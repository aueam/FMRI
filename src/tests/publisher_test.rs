use crate::publisher::Publisher;

#[test]
fn get() {
    assert_eq!(
        Publisher::new("/publisher/".to_owned())
            .unwrap()
            .get_as_string(),
        "publisher"
    );
    assert_eq!(
        Publisher::new("/publisher".to_owned())
            .unwrap()
            .get_as_string(),
        "publisher"
    );
    assert_eq!(
        Publisher::new("publisher/".to_owned())
            .unwrap()
            .get_as_string(),
        "publisher"
    );
    assert_eq!(
        Publisher::new("//publisher//".to_owned())
            .unwrap()
            .get_as_string(),
        "publisher"
    )
}

#[test]
#[should_panic]
fn get_panic() {
    Publisher::new("publ@sher".to_owned()).unwrap();
}

#[test]
fn parse_publisher_from_raw_fmri() {
    let publisher =
        Publisher::parse_publisher_from_raw_fmri("fmri=pkg://publisher/test/test@1-0.1".to_owned())
            .unwrap()
            .unwrap();
    assert_eq!(publisher, Publisher::new("publisher".to_owned()).unwrap());

    let publisher =
        Publisher::parse_publisher_from_raw_fmri("pkg:/test/test@1-0.1".to_owned()).unwrap();
    assert_eq!(publisher, None);
}

#[test]
#[should_panic]
fn parse_publisher_from_raw_fmri_panic() {
    Publisher::parse_publisher_from_raw_fmri("fmri=publisher/pkg://test/test@1-0.1".to_owned())
        .unwrap()
        .unwrap();
    Publisher::parse_publisher_from_raw_fmri("pkg://publisher".to_owned()).unwrap();
}
