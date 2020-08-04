pub mod cli_options {
    use std::path::PathBuf;
    use structopt::StructOpt;

    #[derive(Debug, StructOpt)]
    #[structopt(name = "example", about = "An example of StructOpt usage.")]
    pub struct Opt {
        /// Show version
        #[structopt(short = "V", long = "version")]
        pub version: bool,

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
    use std::io;
    use std::io::prelude::*;
    use std::io::LineWriter;

    pub fn create(l: &[String]) -> Result<(), io::Error> {
        if l.get(0).is_some() {
            let file = File::create("./mrl")?;
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
    use std::fs;
    use std::io;
    use std::io::{Error, ErrorKind};
    use std::os::unix::fs::MetadataExt;
    use std::path::Path;

    fn get_filename(opt_cfg: Option<String>) -> String {
        if opt_cfg.is_none() {
            return "./config_test.conf".to_string();
        }
        opt_cfg.unwrap()
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
        opt_cfg: Option<String>,
        homedir: String,
    ) -> io::Result<(
        Vec<waste::WasteFolderProperties>,
        Vec<configster::OptionProperties>,
    )> {
        let config_file = get_filename(opt_cfg);

        let mut waste_list = Vec::new();

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
