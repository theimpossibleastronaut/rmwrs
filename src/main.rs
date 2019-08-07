use std::fs::rename;
use structopt::StructOpt;
use std::path::PathBuf;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: PathBuf
}

fn main() {

    let _home_dir: Option<PathBuf> = dirs::home_dir();
    println!("Your home directory: {:?}", _home_dir);

    // test code for renaming files
    rename("temp", "temp_foo");

    let args = Cli::from_args();

    // test code for processing cli args
    let content = std::fs::read_to_string(&args.path)
    .expect("could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
