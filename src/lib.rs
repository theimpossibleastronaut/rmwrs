pub mod libgen;

pub mod configster {

    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[derive(Debug)]
    pub struct Value {
        pub primary: String,
        attributes: Vec<String>,
    }

    #[derive(Debug)]
    pub struct Items {
        pub option: String,
        pub value: Value,
    }

    pub fn parse_file(filename: &str) -> Vec<Items> {
        // Open the file in read-only mode (ignoring errors).
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut vec: Vec<Items> = Vec::new();

        for (index, line) in reader.lines().enumerate() {
            let l = line.unwrap();
            let (option, primary_value, attr_vec) = parse_line(&l);
            if option.is_empty() {
                continue;
            }

            let item = Items {
                option: option,
                value: Value {
                    primary: primary_value,
                    attributes: attr_vec,
                },
            };

            vec.push(item);

            // Show the line and its number.
            println!("{}. {}", index + 1, l);
        }
        return vec;
    }

    fn parse_line(l: &str) -> (String, String, Vec<String>) {
        let line = l.trim();
        if line.is_empty() || line.as_bytes()[0] == b'#' {
            return ("".to_string(), "".to_string(), Vec::new());
        }

        let mut option = String::new();
        let mut primary_value = String::new();
        let i = line.find('=');
        match i.is_some() {
            true => {
                option = format!("{}", &line[..i.unwrap()].trim());
                primary_value = format!("{}", &line[i.unwrap() + 1..].trim())
            }
            false => option = line.to_string(),
        }

        (option, primary_value, Vec::new())
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("WASTE = /home/foo"),
            ("WASTE".to_string(), "/home/foo".to_string(), Vec::new())
        );

        assert_eq!(
            parse_line("WASTE=/home/foo"),
            ("WASTE".to_string(), "/home/foo".to_string(), Vec::new())
        );

        assert_eq!(
            parse_line("#WASTE = /home/foo"),
            ("".to_string(), "".to_string(), Vec::new())
        );

        assert_eq!(
            parse_line("        "),
            ("".to_string(), "".to_string(), Vec::new())
        );
    }

    #[test]
    #[ignore]
    fn test_parse_line_FIX_ME() {
        // A case like this needs to be handled. It's an invalid line in the config
        // file. An option can have no value (e.g., DefaultOptionOff), but if there is
        // something after it, it requires an '=' sign.
        assert_eq!(
            parse_line("WASTE  /home/foo"),
            ("".to_string(), "".to_string(), Vec::new())
        );
    }
}
