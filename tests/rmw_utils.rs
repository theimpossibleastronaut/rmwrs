use rmwrs::utils;

#[test]
fn test_escape_url() {
    assert_eq!(utils::escape_url("~/foo/bar&/baz boom"),"~/foo/bar%26/baz%20boom");
    assert_eq!(utils::escape_url("~/hello&there/🐢👀🍻"), "~/hello%26there/🐢👀🍻");
}

#[test]
fn test_unescape_url() {
    assert_eq!(utils::unescape_url("~/foo/bar%26/baz%20boom"), "~/foo/bar&/baz boom");
    assert_eq!(utils::unescape_url("~/hello%26there/🐢👀🍻"),"~/hello&there/🐢👀🍻");
}