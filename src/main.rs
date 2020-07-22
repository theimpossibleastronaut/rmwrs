// use std::option::Option;
// use std::fs::rename;
use std::fs;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long)]
    debug: bool,

    /// Set speed
    // we don't want to name it "speed", need to look smart
    #[structopt(short = "v", long = "version")]
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

    let header = "[TrashInfo]";

    for i in &opt.files {
        let file = Path::new(&i);
        let contents = format!(
            "{}\nPath={}\n",
            header,
            file.canonicalize().unwrap().display()
        );
        let trashinfo_filename = format!("{}{}", i.display(), ".trashinfo");
        fs::write(trashinfo_filename, contents).expect("Error writing to file");
    }
}
