use std::fs::rename;
use std::io;
use structopt::StructOpt;
mod trashinfo;
use rmwrs::cli_options;

fn main() -> Result<(), io::Error> {
    let homedir: String = dirs::home_dir()
        .unwrap_or_default()
        .to_str()
        .unwrap()
        .into();

    let opt = rmwrs::cli_options::Opt::from_args();

    if opt.version {
        cli_options::show_version();
    }

    let (waste_list, _config_vec) = rmwrs::config::parse(opt.custom_config_file, homedir)?;

    let date_now = chrono::Local::now();
    let deletion_date = date_now.format("%Y-%m-%dT%H:%M:%S").to_string();

    let mut renamed_list: Vec<String> = Vec::new();

    // This will be changed later; the subscript number for waste_list depends on whether or not
    // the file being rmw'ed is
    // on the same filesystem as the WASTE folder.
    let waste = &waste_list[0];

    for file in &opt.files {
        let basename = rmwrs::libgen::get_basename(&file).to_str().unwrap();

        let file_absolute = file.canonicalize().unwrap().display().to_string();

        // Will need more error-checking to prevent overwriting existing destination files.
        // As in the C version of rmw, some type of time/date string is appended in that case.
        let destination = format!("{}/{}", &waste.file, basename);
        println!("'{}' -> '{}'", file.display(), destination);
        if rename(file, &destination).is_ok() {
            renamed_list.push(file_absolute.clone());
            let trashinfo_file_contents =
                trashinfo::Trashinfo::new(&file_absolute, &deletion_date).to_contents();

            trashinfo::create(&basename, &waste.info, trashinfo_file_contents)
                .expect("Error writing trashinfo file");
        } else {
            // We don't want to exit the program, just try the next file. In the future
            // we might consider implementing an error counter (e.g. if err > 3
            // then print fatal message && exit).
            println!("Unable to rename {}", file.display());
        }
    }

    // I don't think we need a unit test for mrl file creation; when there's a restore
    // and undo function,
    // it can be tested easily using the bin script test.
    rmwrs::mrl::create(&renamed_list)?;

    Ok(())
}

#[test]
fn test_bin_script() {
    use std::process::Command;

    let status = Command::new("tests/test.sh")
        .args(&[""])
        .status()
        .expect("failed to execute process");

    assert_eq!(status.success(), true);
}
