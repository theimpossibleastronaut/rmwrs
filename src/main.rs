use chrono::Local;
use std::fs::rename;
use std::io;
use std::path::PathBuf;
use structopt::StructOpt;
mod libgen;
mod trashinfo;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long)]
    debug: bool,

    /// Show version
    #[structopt(short = "V", long = "version")]
    version: bool,

    /// Specify path/filename of alternate configuration file
    #[structopt(short = "c", long = "config")]
    custom_config_file: Option<String>,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() -> Result<(), io::Error> {
    let homedir: String = dirs::home_dir()
        .unwrap_or_default()
        .to_str()
        .unwrap()
        .into();

    let opt = Opt::from_args();

    if opt.debug {
        println!("Your home directory: {:?}", homedir);
        println!("{:?}", opt);
    }

    if opt.version {
        println!(
            "{} version: {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        );
        println!();
    }

    let (waste_list, _config_vec) = oxi_rmw::config::parse(opt.custom_config_file, homedir)?;

    let date_now = Local::now();
    let deletion_date = date_now.format("%Y-%m-%dT%H:%M:%S").to_string();

    // This will be changed later; the subscript number for waste_list depends on whether or not
    // the file being rmw'ed is
    // on the same filesystem as the WASTE folder.
    let waste = &waste_list[0];

    // The format of the trashinfo file corresponds to that of the FreeDesktop.org
    // Trash specification<https://specifications.freedesktop.org/trash-spec/trashspec-latest.html>.
    for file in &opt.files {
        let basename = libgen::get_basename(&file).to_str().unwrap();
        let file_absolute = file.canonicalize().unwrap().display().to_string();

        // Will need more error-checking to prevent overwriting existing destination files.
        // As in the C version of rmw, some type of time/date string is appended in that case.
        let destination = format!("{}/{}", &waste.file, basename);
        println!("'{}' -> '{}'", file.display(), destination);
        rename(file, &destination).expect("Error renaming file");

        let contents = trashinfo::create_contents(&file_absolute, &deletion_date);
        trashinfo::create(&basename, &waste.info, contents).expect("Error writing trashinfo file");
    }
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
