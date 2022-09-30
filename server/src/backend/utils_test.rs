#[cfg(test)]
mod path_normalize_tests {
    use crate::utils::validate_and_normalize_path;

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
