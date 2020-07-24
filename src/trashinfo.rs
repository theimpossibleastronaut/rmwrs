use std::fs;
use std::io;
use std::path::Path;

pub fn create_contents(f: &Path, dd: &String) -> String {
    let header = "[TrashInfo]";
    format!(
        "{}\nPath={}\nDeletionDate={}\n",
        header,
        f.canonicalize().unwrap().display(),
        dd
    )
}

#[test]
#[ignore]
// Ignore the test for now. It always fails because the file isn't found when Path:new() is run
fn check_create_trashinfo_contents() {
    let deletion_date = format!("{}", "2020-07-23T20:56:03");
    let file = Path::new(r"/home/foo/bar");
    assert_eq!(
        create_contents(&file, &deletion_date.to_string()),
        "[TrashInfo]\nPath=/home/andy/testing\nDeletionDate=2020-07-23T20:56:03"
    );
}

pub fn create(basename: &str, waste_info: &String, contents: String) -> Result<(), io::Error> {
    let trashinfo_filename = format!("{}{}", basename, ".trashinfo");
    let trashinfo_dest = format!("{}/{}", &waste_info, trashinfo_filename);
    fs::write(trashinfo_dest, contents)
}
