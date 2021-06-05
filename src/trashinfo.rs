use std::fs;
use std::io;
use rmwrs::utils::{percent_encode, percent_decode, read_lines};

pub struct Trashinfo(String, pub String, pub String);

/// The format of the trashinfo file corresponds to that of the FreeDesktop.org
/// Trash specification<https://specifications.freedesktop.org/trash-spec/trashspec-latest.html>.
impl Trashinfo {
    pub fn new(path_and_filename: &str, deletion_date: &str) -> Self {
        Self {
            0: "[Trash Info]".to_string(),
            1: path_and_filename.to_string(),
            2: deletion_date.to_string(),
        }
    }
    
    pub fn to_contents(&self) -> String {
        let pct_string = percent_encode(&self.1);
        format!("{}\nPath={}\nDeletionDate={}\n", self.0, pct_string, self.2)
    }

    pub fn from_file(path: &str) -> io::Result<Trashinfo> {
        let mut lines = read_lines(path)?.skip(1);
        let mut path_and_filename = lines.next().expect("Failed to read the path in trashinfo.")?;
        path_and_filename.replace_range(..5, "");
        path_and_filename = percent_decode(&path_and_filename).expect("Could not decode path in trashinfo.");
        let mut deletion_date = lines.next().expect("Failed to read deletion date in trashinfo.")?;
        deletion_date.replace_range(..13, "");
        Ok(Trashinfo::new(&path_and_filename, &deletion_date))
    }
}

#[test]
fn check_create_trashinfo_contents() {
    let deletion_date = format!("{}", "2020-07-23T20:56:03".to_string());
    let file: &str = "/home/foo/bar";
    assert_eq!(
        Trashinfo::new(&file, &deletion_date).to_contents(),
        "[Trash Info]\nPath=/home/foo/bar\nDeletionDate=2020-07-23T20:56:03\n"
    );
}

pub fn create(basename: &str, waste_info: &str, contents: String) -> io::Result<()> {
    let trashinfo_filename = format!("{}{}", basename, ".trashinfo");
    let trashinfo_dest = format!("{}/{}", &waste_info, trashinfo_filename);
    fs::write(trashinfo_dest, contents)
}

pub fn info_path(file_to_restore_path: &str) -> String {
    // This is a bit lazy and doesn't bother reading the waste_info.
    format!("{}.trashinfo", file_to_restore_path).replace("/files/", "/info/")
}

#[test]
fn test_info_path() {
    assert_eq!("~/.hello&there/info/🐢👀🍻.trashinfo", info_path("~/.hello&there/files/🐢👀🍻"))
}
