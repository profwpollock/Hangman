#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

mod unit_tests {
    //! Unit tests for hangman.
    use crate::*;

    #[test]
    #[ignore = "dummy test"]
    fn dummy_test() {
        assert_eq!(2+2, 4);
    }

    #[test]
    fn test_asset_path () {
        println!("The path to the assets directory is {}",
        get_assets_directory().display());
    }

    #[test]
    #[should_panic]
    fn test_err () {
        err("this is a test of the err function");
    }
}
