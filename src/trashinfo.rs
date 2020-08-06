use std::fs;
use std::io;

/// The format of the trashinfo file corresponds to that of the FreeDesktop.org
/// Trash specification<https://specifications.freedesktop.org/trash-spec/trashspec-latest.html>.
pub fn create_contents(path_and_filename: &str, deletion_date: &str) -> String {
    format!(
        "{}\nPath={}\nDeletionDate={}\n",
        "[TrashInfo]".to_string(),
        path_and_filename.to_string(),
        deletion_date.to_string()
    )
}

#[test]
fn check_create_trashinfo_contents() {
    let deletion_date = format!("{}", "2020-07-23T20:56:03".to_string());
    let file: &str = "/home/foo/bar";
    assert_eq!(
        create_contents(&file, &deletion_date),
        "[TrashInfo]\nPath=/home/foo/bar\nDeletionDate=2020-07-23T20:56:03\n"
    );
}

pub fn create(basename: &str, waste_info: &str, contents: String) -> Result<(), io::Error> {
    let trashinfo_filename = format!("{}{}", basename, ".trashinfo");
    let trashinfo_dest = format!("{}/{}", &waste_info, trashinfo_filename);
    fs::write(trashinfo_dest, contents)
}
