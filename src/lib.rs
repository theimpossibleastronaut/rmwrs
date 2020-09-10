use std::env;
use std::io::{self, ErrorKind};

pub fn get_homedir() -> io::Result<String> {
    let homedir: String = match env::var("RMWRS_TEST_HOME") {
        Ok(val) => val,
        Err(_e) => match dirs::home_dir() {
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
    let data_home: String = match env::var("XDG_DATA_HOME") {
        Ok(val) => val,
        Err(_e) => format!("{}{}", homedir, "/.local/share").to_string(),
    };
    let datadir = data_home + "/rmwrs";
    if !std::path::Path::new(&datadir).exists() {
        println!("Creating {}", &datadir);
        std::fs::create_dir_all(&datadir).expect("Unable to create config directory");
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

pub mod config {

    use crate::waste;
    use std::env;
    use std::fs;
    use std::io::{self, Error, ErrorKind};
    use std::os::unix::fs::MetadataExt;
    use std::path::Path;

    pub fn get_filename(homedir: &str, opt_cfg: Option<String>) -> String {
        let config_home;
        if opt_cfg.is_none() {
            config_home = match env::var("XDG_CONFIG_HOME") {
                Ok(val) => val,
                Err(_e) => format!("{}{}", homedir, "/.config").to_string(),
            };
            if !Path::new(&config_home).exists() {
                println!("Creating {}", &config_home);
                fs::create_dir_all(&config_home).expect("Unable to create config directory");
            }
            return config_home + "/rmwrsrc";
        } else {
            return opt_cfg.unwrap();
        }
    }

    fn get_dev_num(wp: &str) -> io::Result<u64> {
        // Device num not used yet. Used to determine what file system a file is on,
        // and therefore which waste folder it can be rmw'ed to. (rmw doesn't move or
        // copy files to different file systems. (Apparently not available on Windows:
        // https://doc.rust-lang.org/std/os/windows/fs/trait.MetadataExt.html)
        let meta = fs::metadata(wp)?;
        Ok(meta.dev())
    }

    fn assign_properties(
        st_option_props: &configster::OptionProperties,
        homedir: &str,
    ) -> io::Result<waste::WasteFolderProperties> {
        let mut waste_properties = waste::WasteFolderProperties::new();
        waste_properties.parent = st_option_props.value.primary.replace("$HOME", &homedir);

        waste_properties.info = format!("{}{}", waste_properties.parent, "/info");
        println!("Using {}", &waste_properties.info);
        if !Path::new(&waste_properties.info).exists() {
            println!("Creating {}", &waste_properties.info);
            fs::create_dir_all(&waste_properties.info)?;
        }

        waste_properties.file = format!("{}{}", waste_properties.parent, "/files");
        println!("Using {}", &waste_properties.file);
        if !Path::new(&waste_properties.file).exists() {
            println!("Creating {}", &waste_properties.file);
            fs::create_dir_all(&waste_properties.file)?;
        }
        Ok(waste_properties)
    }

    fn is_removable(v_attrs: &[String]) -> io::Result<bool> {
        match v_attrs.get(0).is_some() {
            true => {
                if v_attrs[0] == "removable" {
                    Ok(true)
                } else {
                    io::Result::Err(Error::new(
                        ErrorKind::InvalidData,
                        "Unknown attribute (try 'removable'",
                    ))
                }
            }
            false => Ok(false),
        }
    }

    pub fn parse(
        homedir: &str,
        config_file: &str,
    ) -> io::Result<(
        Vec<waste::WasteFolderProperties>,
        Vec<configster::OptionProperties>,
    )> {
        let mut waste_list = Vec::new();

        if !std::path::Path::new(&config_file).exists() {
            fs::write(
                &config_file,
                "WASTE = $HOME/.rmwrs-Trash-test,removable\npurge_after = 90\nforce_required",
            )?;
        }

        let config_vec = configster::parse_file(&config_file, ',')?;

        // This code will get replaced by a match statement later as we'll
        // be adding more configuration options, such as 'purge_after' and
        // 'force_required'
        for st_i in &config_vec {
            if st_i.option == "WASTE" {
                let mut waste_properties = assign_properties(&st_i, &homedir)?;

                waste_properties.is_removable = is_removable(&st_i.value.attributes)?;

                waste_properties.dev_num = get_dev_num(&waste_properties.parent)?;

                waste_list.push(waste_properties);
            }
        }

        Ok((waste_list, config_vec))
    }
}
