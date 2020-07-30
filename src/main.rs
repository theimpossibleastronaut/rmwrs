// use std::option::Option;
use std::fs::rename;
// use std::fmt::Display;
use chrono::Local;
use std::fs;
use std::io;
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

    let config_file: String;
    if opt.custom_config_file.is_none() {
        config_file = "./config_test.conf".to_string();
    } else {
        config_file = opt.custom_config_file.unwrap();
    }

    let config_vec = oxi_rmw::configster::parse_file(&config_file, ',');
    if config_vec.is_err() {
        return io::Result::Err(config_vec.unwrap_err());
    }

    let mut waste_list = Vec::new();

    for i in &config_vec.unwrap() {
        if i.option == "WASTE" {
            let mut waste_properties = oxi_rmw::waste::WasteFolderProperties::new();
            waste_properties.parent = i.value.primary.replace("$HOME", &homedir);

            waste_properties.info = format!("{}{}", waste_properties.parent, "/info");
            println!("Using {}", &waste_properties.info);
            if Path::new(&waste_properties.info).exists() == false {
                println!("Creating {}", &waste_properties.info);
                let r = fs::create_dir_all(&waste_properties.info);
                if r.is_err() {
                    return io::Result::Err(r.unwrap_err());
                }
            }

            waste_properties.file = format!("{}{}", waste_properties.parent, "/files");
            println!("Using {}", &waste_properties.file);
            if Path::new(&waste_properties.file).exists() == false {
                println!("Creating {}", &waste_properties.file);
                let r = fs::create_dir_all(&waste_properties.file);
                if r.is_err() {
                    return io::Result::Err(r.unwrap_err());
                }
            }

            if i.value.attributes[0] == "removable".to_string() {
                waste_properties.is_removable = true;
            }

            waste_list.push(waste_properties);
        }
    }

    let date_now = Local::now();
    let deletion_date = date_now.format("%Y-%m-%dT%H:%M:%S").to_string();

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
