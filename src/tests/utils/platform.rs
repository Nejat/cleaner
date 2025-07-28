use crate::utils::path_of_supported_platforms_configuration;

#[test]
fn given_a_request_for_configuration_path_the_path_should_point_to_configuration_json() {
    const EXPECTED: &str = "supported-platforms.json";

    let actual = path_of_supported_platforms_configuration()
        .to_string_lossy()
        .to_string();

    assert_eq!(EXPECTED, &actual[actual.len() - EXPECTED.len()..]);
}
