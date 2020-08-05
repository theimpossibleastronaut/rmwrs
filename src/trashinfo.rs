use std::fs;
use std::io;

/// The format of the trashinfo file corresponds to that of the FreeDesktop.org
/// Trash specification<https://specifications.freedesktop.org/trash-spec/trashspec-latest.html>.
pub struct Trashinfo(String, String, String);

impl Trashinfo {
    pub fn new(path_and_filename: &str, deletion_date: &str) -> Self {
        Self {
            0: "[TrashInfo]".to_string(),
            1: path_and_filename.to_string(),
            2: deletion_date.to_string(),
        }
    }
    pub fn to_contents(&self) -> String {
        format!("{}\nPath={}\nDeletionDate={}\n", self.0, self.1, self.2)
    }
}

#[test]
fn check_create_trashinfo_contents() {
    let deletion_date = format!("{}", "2020-07-23T20:56:03".to_string());
    let file: &str = "/home/foo/bar";
    assert_eq!(
        Trashinfo::new(&file, &deletion_date).to_contents(),
        "[TrashInfo]\nPath=/home/foo/bar\nDeletionDate=2020-07-23T20:56:03\n"
    );
}

pub fn create(basename: &str, waste_info: &str, contents: String) -> Result<(), io::Error> {
    let trashinfo_filename = format!("{}{}", basename, ".trashinfo");
    let trashinfo_dest = format!("{}/{}", &waste_info, trashinfo_filename);
    fs::write(trashinfo_dest, contents)
}
