use std::fs::rename;
use std::io;
use structopt::StructOpt;
mod trashinfo;
use rmwrs::cli_options;
mod config;

fn main() -> Result<(), io::Error> {
    let homedir = rmwrs::get_homedir()?;
    let datadir = rmwrs::get_datadir(&homedir);

    let opt = rmwrs::cli_options::Opt::from_args();

    if opt.verbose {
        println!("homedir: '{}'", homedir);
    }

    if opt.version {
        cli_options::show_version();
    }

    let config_file = config::get_filename(&homedir, opt.custom_config_file);

    let (waste_list, _config_vec) = config::load(&homedir, &config_file)?;

    let date_now = chrono::Local::now();
    let deletion_date = date_now.format("%Y-%m-%dT%H:%M:%S").to_string();
    let noclobber_suffix = date_now.format("_%H%M%S-%y%m%d").to_string();

    
    // This will be changed later; the subscript number for waste_list depends on whether or not
    // the file being rmw'ed is
    // on the same filesystem as the WASTE folder.
    let waste = &waste_list[0];
    
    for file in &opt.files {
        let file_absolute: Option<String> = file.canonicalize().map_or_else(
            |e| {
                println!("{}", e);
                None
            },
            |v| Some(v.display().to_string()),
        );
        if file_absolute == None {
            continue;
        }
        
        let mut basename = rmwrs::libgen::get_basename(&file)
            .to_str()
            .unwrap()
            .to_owned();
        
        if opt.restore {
            let info_path = trashinfo::info_path(&file_absolute.unwrap());
            let trash_info = trashinfo::Trashinfo::from_file(&info_path)?;
            let destination = trash_info.1.clone();
            
            match rename(file, &trash_info.1) {
                Ok(_val) => {
                    println!("'{}' -> '{}'", file.display(), destination);
                    std::fs::remove_file(info_path)?;
                }
                Err(e) => println!("Error {} renaming {}", e, file.display()),
            }
        }
        else {
            let mut renamed_list: Vec<String> = Vec::new();
            let mut destination = format!("{}/{}", &waste.file, basename).to_owned();

            if std::path::Path::new(&destination).exists() {
                basename.push_str(&noclobber_suffix);
                destination.push_str(&noclobber_suffix);
            }
    
            match rename(&file, &destination) {
                Ok(_val) => {
                    println!("'{}' -> '{}'", file.display(), destination);
                    renamed_list.push(destination.clone());
                    let trashinfo_file_contents =
                        trashinfo::Trashinfo::new(&file_absolute.unwrap(), &deletion_date)
                            .to_contents();
    
                    trashinfo::create(&basename, &waste.info, trashinfo_file_contents)
                        .expect("Error writing trashinfo file");
                }
                Err(e) => println!("Error {} renaming {}", e, file.display()),
            }
            // I don't think we need a unit test for mrl file creation; when there's a restore
            // and undo function,
            // it can be tested easily using the bin script test.
            rmwrs::mrl::create(&datadir, &renamed_list)?;
        }
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
