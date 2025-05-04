use lines_counter::parse_config::Config;

#[test]
fn test_config_with_valid_path() {
    let args = [
        "lines_counter".to_string(),
        "./".to_string(),
    ];
    let config = Config::new(&args).unwrap();
    assert!(!config.is_recursive);
}

#[test]
fn test_config_with_valid_path_and_recursive_flag() {
    let args = [
        "lines_counter".to_string(),
        "-r".to_string(),
        "./".to_string(),
    ];
    let config = Config::new(&args).unwrap();
    assert!(config.is_recursive);
}

#[test]
fn test_config_with_invalid_path() {
    let args = [
        "lines_counter".to_string(),
        "non_existing_path".to_string(),
    ];

    let config = Config::new(&args);
    assert!(config.is_err());
}

#[test]
fn test_config_with_invalid_argument_count() {
    let args = [
        "lines_counter".to_string(),
    ];

    let config = Config::new(&args);
    assert!(config.is_err());
}