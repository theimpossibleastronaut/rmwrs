use std::env;
use std::io::{self, ErrorKind};
pub mod utils;

pub fn get_homedir() -> io::Result<String> {
    let homedir: String = match env::var("RMWRS_TEST_HOME") {
        Ok(val) => val,
        Err(_e) => match home::home_dir() {
            None => {
                return Err(io::Error::new(
                    ErrorKind::NotFound,
                    "Unable to determine homedir",
                ))
            }
            Some(homedir) => homedir.to_str().unwrap().into(),
        },
    };
    Ok(homedir)
}

pub fn get_datadir(homedir: &str) -> String {
    let default = homedir.to_string() + "/.local/share";
    let data_home: String;
    if env::var("RMWRS_TEST_HOME").is_err() {
        // Don't use $XDG_DATA_HOME if rmwrs is in test mode
        data_home = default;
    } else {
        data_home = match env::var("XDG_DATA_HOME") {
            Ok(val) => val,
            Err(_e) => default,
        };
    }
    let datadir = data_home + "/rmwrs";
    if !std::path::Path::new(&datadir).exists() {
        println!("Creating {}", &datadir);
        std::fs::create_dir_all(&datadir).expect("Unable to create data directory");
    }
    datadir
}

pub mod cli_options {
    use std::path::PathBuf;
    use structopt::StructOpt;

    #[derive(Debug, StructOpt)]
    pub struct Opt {
        /// Show version
        #[structopt(short = "V", long = "version")]
        pub version: bool,

        #[structopt(short = "v", long = "verbose")]
        pub verbose: bool,

        /// Specify path/filename of alternate configuration file
        #[structopt(short = "c", long = "config")]
        pub custom_config_file: Option<String>,

        /// Restore files
        #[structopt(short = "z", long = "restore")]
        pub restore: bool,

        /// Files to process
        #[structopt(name = "FILE", parse(from_os_str))]
        pub files: Vec<PathBuf>,
    }

    pub fn show_version() {
        println!(
            "{} version: {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        );
        println!();
    }
}

pub mod libgen {
    use std::ffi::OsStr;
    use std::path::Path;

    pub fn get_basename(path: &Path) -> &OsStr {
        path.file_name().unwrap_or_else(|| path.as_os_str())
    }
}

pub mod mrl {

    use std::fs::File;
    use std::io::{self, prelude::*, LineWriter};

    pub fn create(datadir: &str, l: &[String]) -> Result<(), io::Error> {
        if l.get(0).is_some() {
            let file = File::create(datadir.to_string() + "/mrl")?;
            let mut file = LineWriter::new(file);
            for i in l {
                file.write_all((i.clone() + "\n").as_bytes())?;
            }
            file.flush()?
        }
        Ok(())
    }
}

pub mod waste {
    pub struct WasteFolderProperties {
        pub parent: String,
        pub info: String,
        pub file: String,
        pub is_removable: bool,
        pub dev_num: u64,
    }

    impl Default for WasteFolderProperties {
        fn default() -> Self {
            Self::new()
        }
    }

    impl WasteFolderProperties {
        pub fn new() -> Self {
            Self {
                parent: String::new(),
                info: String::new(),
                file: String::new(),
                is_removable: false,
                dev_num: 0,
            }
        }
    }
}
