use rmwrs::utils;

#[test]
fn test_escape_url() {
    assert_eq!(utils::escape_url(&"~/foo/bar&/baz boom".to_owned()),"~/foo/bar%26/baz%20boom");
}