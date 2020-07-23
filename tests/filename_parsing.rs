use oxi_rmw::libgen;
use std::path::PathBuf;

#[test]
fn check_get_basename() {
    let mut some_basename = PathBuf::from(r"./test/basename/");
    assert_eq!(libgen::get_basename(&some_basename), "basename");

    some_basename = PathBuf::from(r"/./test/basename");
    assert_eq!(libgen::get_basename(&some_basename), "basename");

    // a filename containing spaces
    some_basename = PathBuf::from(r"/./test/base name/");
    assert_eq!(libgen::get_basename(&some_basename), "base name");
}

#[test]
#[ignore]
fn check_get_basename_windows() {
    // a file on Windows
    let some_basename = PathBuf::from(r"C:\windows\system32.dll");
    // Does not work on Linux (and maybe not on Windows either; returns "C:\\windows\\system32.dll")
    assert_eq!(libgen::get_basename(&some_basename), "system32.dll");
}
