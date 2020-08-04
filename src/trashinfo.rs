use std::fs;
use std::io;

pub fn create_contents(f: &str, dd: &str) -> String {
    let header = "[TrashInfo]";
    format!("{}\nPath={}\nDeletionDate={}\n", header, f, dd)
}

#[test]
fn check_create_trashinfo_contents() {
    let deletion_date = format!("{}", "2020-07-23T20:56:03".to_string());
    let file: &str = "/home/foo/bar";
    assert_eq!(
        create_contents(&file, &deletion_date.to_string()),
        "[TrashInfo]\nPath=/home/foo/bar\nDeletionDate=2020-07-23T20:56:03\n"
    );
}

pub fn create(basename: &str, waste_info: &str, contents: String) -> Result<(), io::Error> {
    let trashinfo_filename = format!("{}{}", basename, ".trashinfo");
    let trashinfo_dest = format!("{}/{}", &waste_info, trashinfo_filename);
    fs::write(trashinfo_dest, contents)
}
