#[cfg(test)]
mod path_normalize_tests {
    use crate::backend::utils::validate_and_normalize_path;

    fn validation_works(before: &str, after: &str) {
        println!("Works: --- {} ------------", before);
        let result = validate_and_normalize_path(before.to_string()).unwrap();

        println!("Result: {}", result.display());
        assert_eq!(result.to_string_lossy(), after);
    }

    fn validation_fails(before: &str) {
        println!("Fails: --- {} ------------", before);
        let result = validate_and_normalize_path(before.to_string());

        assert!(result.is_err());
    }

    #[test]
    fn test_path_normalization() {
        validation_works("/abc/test/../thing.png", "/abc/thing.png");
        validation_works("/abc/def/../../thing.png", "/thing.png");
        validation_works("/home/dys/test", "/home/dys/test");
        validation_works("/home/dys", "/home/dys");
        validation_works("/home/dys/", "/home/dys/");
        validation_works("/home/dys/..", "/home");
        validation_works("/home/dys/../", "/home/");
        validation_works("π/2", "π/2");
        validation_works(
            "/home/dys/dev/broot/../../../canop/test",
            "/home/canop/test",
        );
        validation_works("/.", "/");
        validation_works("/./", "/");
        // fails
        validation_fails("./test");
        validation_fails("../test");
        validation_fails("/abc/../../thing.png");
        validation_fails("/home/dys/../../../test");
        validation_fails("/..");
        validation_fails("../");
    }
}

#[cfg(test)]
mod rclone_modification_check_tests {
    use chrono::NaiveDateTime;
    use std::{path::PathBuf, str::FromStr};

    use crate::backend::utils::path_contains_rclone_modification_date;

    #[test]
    fn timestamp_parsing_works() {
        let result = NaiveDateTime::parse_from_str("20221003093709", "%Y%m%d%H%M%S").unwrap();
        let resulting_string = result.to_string();
        assert_eq!("2022-10-03 09:37:09", resulting_string)
    }

    #[test]
    fn path_contains_rclone_modification_date_works() {
        let path = PathBuf::from_str("/20221003093709 /Home/School").unwrap();
        let option = path_contains_rclone_modification_date(&path);

        match option {
            Some(result) => {
                assert_eq!(
                    NaiveDateTime::parse_from_str("20221003093709", "%Y%m%d%H%M%S").unwrap(),
                    result.0
                );
                assert_eq!(PathBuf::from_str("/Home/School").unwrap(), result.1);
            }
            None => panic!("Expected some value here."),
        }
    }

    #[test]
    fn path_contains_rclone_modification_date_fails_without_whitespace() {
        let path = PathBuf::from_str("/20221003093709/Home/School").unwrap();
        let option = path_contains_rclone_modification_date(&path);
        assert!(option.is_none())
    }

    #[test]
    fn path_contains_rclone_modification_date_fails_with_wrong_timestamp_format() {
        let path = PathBuf::from_str("/202210030937 /Home/School").unwrap();
        let option = path_contains_rclone_modification_date(&path);
        assert!(option.is_none())
    }

    #[test]
    fn path_contains_rclone_modification_date_fails_with_wrong_timestamp() {
        let path = PathBuf::from_str("/20221003093790 /Home/School").unwrap();
        let option = path_contains_rclone_modification_date(&path);
        assert!(option.is_none())
    }
}
