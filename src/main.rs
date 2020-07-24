// use std::option::Option;
use std::fs::rename;
// use std::fmt::Display;
use chrono::Local;
use std::fs;
use std::path::{Path, PathBuf};
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

    /// Set speed
    // we don't want to name it "speed", need to look smart
    #[structopt(short = "V", long = "version")]
    version: bool,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() {
    // https://github.com/openethereum/openethereum/pull/9077/files
    // https://doc.rust-lang.org/std/option/enum.Option.html#variant.Some
    /* "Because unwrap() may panic, its use is generally
    discouraged. Instead, prefer to use pattern matching and handle the
    None case explicitly, or call unwrap_or, unwrap_or_else, or
    unwrap_or_default. */
    let homedir: String = dirs::home_dir()
        .unwrap_or_default()
        .to_str()
        .unwrap()
        .into();

    println!("Your home directory: {:?}", homedir);

    // test code for renaming files
    // rename("temp", "temp_foo");

    let opt = Opt::from_args();
    println!("{:?}", opt);
    if opt.version {
        println!(
            "{} version: {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        );
        println!();
    }

    let waste_info = format!("{}/{}", homedir, ".oxi-rmw-Trash-test/info");
    println!("Using {}", &waste_info);
    if Path::new(&waste_info).exists() == false {
        println!("Creating {}", &waste_info);
        fs::create_dir_all(&waste_info).expect("Could not create directory");
    }

    let waste_files = format!("{}/{}", homedir, ".oxi-rmw-Trash-test/files");
    println!("Using {}", &waste_files);
    if Path::new(&waste_files).exists() == false {
        println!("Creating {}", &waste_files);
        fs::create_dir_all(&waste_files).expect("Could not create directory");
    }

    let date_now = Local::now();
    let deletion_date = date_now.format("%Y-%m-%dT%H:%M:%S");

    // The format of the trashinfo file corresponds to that of the FreeDesktop.org
    // Trash specification<https://specifications.freedesktop.org/trash-spec/trashspec-latest.html>.
    for i in &opt.files {
        let file = Path::new(&i);
        let contents = trashinfo::create_trashinfo_contents(file, &deletion_date.to_string());

        let basename = libgen::get_basename(&i).to_str().unwrap();
        let trashinfo_filename = format!("{}{}", basename, ".trashinfo");
        let trashinfo_dest = format!("{}/{}", &waste_info, trashinfo_filename);
        fs::write(trashinfo_dest, contents).expect("Error writing to file");

        // Will need more error-checking to prevent overwriting existing destination files.
        // As in the C version of rmw, some type of time/date string is appended in that case.

        // BUG: This doesn't work unless the files are in the current directory
        // (e.g. 'cargo run -- foo bar` will work, but not 'cargo run -- tmp/foo tmp/bar'
        let destination = format!("{}/{}", &waste_files, basename);
        rename(i, &destination).expect("Error renaming file");
    }
}
