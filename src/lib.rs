pub mod libgen;

pub mod configster {

    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[derive(Debug)]
    struct Value {
        primary: String,
        attributes: Vec<String>,
    }

    #[derive(Debug)]
    pub struct Items {
        key: String,
        value: Value,
    }

    pub fn parse_file() -> Vec<Items> {
        let filename = "./config_test.conf";
        // Open the file in read-only mode (ignoring errors).
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut vec: Vec<Items> = Vec::new();
        // Read the file line by line using the lines() iterator from std::io::BufRead.

        let mut count = 0;

        for (index, line) in reader.lines().enumerate() {
            let l = line.unwrap(); // Ignore errors.
            let line = l.trim();

            if line.is_empty() {
                continue;
            }

            if line.as_bytes()[0] == b'#' {
                continue;
            }

            let i = line.find('=');

            let mut key = String::new();
            let mut primary_value = String::new();

            match i.is_some() {
                true => {
                    key = format!("{}", &line[..i.unwrap()].trim());
                    primary_value = format!("{}", &line[i.unwrap() + 1..].trim())
                }
                false => key = line.to_string(),
            }

            let attr_vec: Vec<String> = Vec::new();

            let item = Items {
                key: key,
                value: Value {
                    primary: primary_value,
                    attributes: attr_vec,
                },
            };

            vec.push(item);

            // Show the line and its number.
            println!("{}. {}", index + 1, line);
        }
        return vec;
    }
}
