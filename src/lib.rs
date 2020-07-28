pub mod libgen;

pub mod configster {

    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[derive(Debug)]
    pub struct Value {
        pub primary: String,
        pub attributes: Vec<String>,
    }

    #[derive(Debug)]
    pub struct Items {
        pub option: String,
        pub value: Value,
    }

    /// Parses a configuration file. The second parameter sets the delimiter for the
    /// attribute list of the primary value.
    pub fn parse_file(filename: &str, attr_delimit_char: char) -> Vec<Items> {
        // Open the file in read-only mode (ignoring errors).
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut vec: Vec<Items> = Vec::new();

        for (index, line) in reader.lines().enumerate() {
            let l = line.unwrap();
            let (option, primary_value, attr_vec) = parse_line(&l, attr_delimit_char);
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

    fn parse_line(l: &str, attr_delimit_char: char) -> (String, String, Vec<String>) {
        let line = l.trim();
        if line.is_empty() || line.as_bytes()[0] == b'#' {
            return ("".to_string(), "".to_string(), Vec::new());
        }

        let option;
        let mut value = String::new();
        let mut i = line.find('=');
        match i.is_some() {
            true => {
                option = format!("{}", &line[..i.unwrap()].trim());
                value = format!("{}", &line[i.unwrap() + 1..].trim())
            }
            false => option = line.to_string(),
        }

        i = value.find(attr_delimit_char);
        let primary_value;
        let mut tmp_attr_vec: Vec<&str> = Vec::new();
        let attributes;
        match i.is_some() {
            true => {
                primary_value = format!("{}", &value[..i.unwrap()].trim());
                attributes = format!("{}", &value[i.unwrap() + 1..]);
                tmp_attr_vec = attributes.split(attr_delimit_char).collect();
            }
            false => primary_value = format!("{}", value.to_string()),
        }

        let mut attr_vec: Vec<String> = Vec::new();
        for a in &tmp_attr_vec {
            attr_vec.push(a.trim().to_string());
        }

        (option, primary_value, attr_vec)
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("WASTE = /home/foo", ','),
            ("WASTE".to_string(), "/home/foo".to_string(), Vec::new())
        );

        assert_eq!(
            parse_line("WASTE=/home/foo", ','),
            ("WASTE".to_string(), "/home/foo".to_string(), Vec::new())
        );

        assert_eq!(
            parse_line("#WASTE = /home/foo", ','),
            ("".to_string(), "".to_string(), vec![])
        );

        assert_eq!(
            parse_line("WASTE = /home/foo, removable, test", ','),
            (
                "WASTE".to_string(),
                "/home/foo".to_string(),
                vec!["removable".to_string(), "test".to_string()]
            )
        );

        assert_eq!(
            parse_line("        ", ','),
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
            parse_line("WASTE  /home/foo", '='),
            ("".to_string(), "".to_string(), Vec::new())
        );
    }
}
