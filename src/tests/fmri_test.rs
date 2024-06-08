use crate::FMRI;

#[test]
fn compare() {
    assert_ne!(
        FMRI::parse_raw(&"test@1,1-1:20220913T082027Z".to_owned())
            .unwrap()
            .version,
        FMRI::parse_raw(&"test@1-1:20220913T082027Z".to_owned())
            .unwrap()
            .version
    );
    assert_eq!(
        FMRI::parse_raw(&"test".to_owned()).unwrap().version,
        FMRI::parse_raw(&"test".to_owned()).unwrap().version
    );
    assert_ne!(
        FMRI::parse_raw(&"test@1".to_owned()).unwrap().version,
        FMRI::parse_raw(&"test".to_owned()).unwrap().version
    );
    assert!(
        FMRI::parse_raw(&"test@2".to_owned()).unwrap().version
            > FMRI::parse_raw(&"test@1".to_owned()).unwrap().version
    );
    assert!(
        FMRI::parse_raw(&"test@1".to_owned()).unwrap().version
            < FMRI::parse_raw(&"test@2".to_owned()).unwrap().version
    );
}
