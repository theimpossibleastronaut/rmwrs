use rmwrs::waste;
use std::env;
use std::fs;
use std::io::{self, Error, ErrorKind};
use std::os::unix::fs::MetadataExt;
use std::path::Path;

pub fn get_filename(homedir: &str, opt_cfg: Option<String>) -> String {
    let default = homedir.to_string() + "/.config";
    let config_home: String;
    if opt_cfg.is_none() {
        if env::var("RMWRS_TEST_HOME").is_err() {
            // Don't use $XDG_CONFIG_HOME if rmwrs is in test mode
            config_home = default
        } else {
            config_home = match env::var("XDG_CONFIG_HOME") {
                Ok(val) => val,
                Err(_e) => default,
            };
        }
    } else {
        return opt_cfg.unwrap();
    }
    if !Path::new(&config_home).exists() {
        println!("Creating {}", &config_home);
        fs::create_dir_all(&config_home).expect("Unable to create config directory");
    }
    return config_home + "/rmwrsrc";
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

pub fn load(
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
