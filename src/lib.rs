pub mod libgen;

pub mod waste {
    pub struct WasteFolderProperties {
        pub parent: String,
        pub info: String,
        pub file: String,
        pub is_removable: bool,
        pub dev_num: u64,
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
    use std::os::unix::fs::MetadataExt;
    use std::path::Path;

    fn get_filename(opt_cfg: Option<String>) -> String {
        if opt_cfg.is_none() {
            return "./config_test.conf".to_string();
        }
        opt_cfg.unwrap()
    }

    fn get_dev_num(wp: &String) -> io::Result<u64> {
        // Device num not used yet. Used to determine what file system a file is on,
        // and therefore which waste folder it can be rmw'ed to. (rmw doesn't move or
        // copy files to different file systems. (Apparently not available on Windows:
        // https://doc.rust-lang.org/std/os/windows/fs/trait.MetadataExt.html)
        let meta = fs::metadata(wp)?;
        Ok(meta.dev())
    }

    fn assign_properties(
        st_option_props: &configster::OptionProperties,
        homedir: &String,
    ) -> io::Result<waste::WasteFolderProperties> {
        let mut waste_properties = waste::WasteFolderProperties::new();
        waste_properties.parent = st_option_props.value.primary.replace("$HOME", &homedir);

        waste_properties.info = format!("{}{}", waste_properties.parent, "/info");
        println!("Using {}", &waste_properties.info);
        if Path::new(&waste_properties.info).exists() == false {
            println!("Creating {}", &waste_properties.info);
            fs::create_dir_all(&waste_properties.info)?;
        }

        waste_properties.file = format!("{}{}", waste_properties.parent, "/files");
        println!("Using {}", &waste_properties.file);
        if Path::new(&waste_properties.file).exists() == false {
            println!("Creating {}", &waste_properties.file);
            fs::create_dir_all(&waste_properties.file)?;
        }
        Ok(waste_properties)
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

        for st_i in &config_vec {
            if st_i.option == "WASTE" {
                let mut waste_properties = assign_properties(&st_i, &homedir)?;

                if st_i.value.attributes[0] == "removable".to_string() {
                    waste_properties.is_removable = true;
                }

                waste_properties.dev_num = get_dev_num(&waste_properties.parent)?;

                waste_list.push(waste_properties);
            }
        }

        Ok((waste_list, config_vec))
    }
}
