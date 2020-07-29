pub mod libgen;

pub mod configster {

    use std::fs::File;
    use std::io;
    use std::io::{BufRead, BufReader};

    #[derive(Debug)]
    pub struct Value {
        pub primary: String,
        pub attributes: Vec<String>,
    }

    #[derive(Debug)]
    pub struct OptionProperties {
        pub option: String,
        pub value: Value,
    }

    impl OptionProperties {
        fn new(option: String, primary: String, attributes: Vec<String>) -> Self {
            Self {
                option,
                value: Value {
                    primary,
                    attributes,
                },
            }
        }
    }

    /// Parses a configuration file. The second parameter sets the delimiter for the
    /// attribute list of the primary value.
    pub fn parse_file(
        filename: &str,
        attr_delimit_char: char,
    ) -> io::Result<Vec<OptionProperties>> {
        let file = File::open(filename);
        if file.is_err() {
            return io::Result::Err(file.unwrap_err());
        }

        let reader = BufReader::new(file.unwrap());
        let mut vec: Vec<OptionProperties> = Vec::new();

        for (index, line) in reader.lines().enumerate() {
            let l = line.unwrap();

            // Parse the line, return the properties
            let (option, primary_value, attr_vec) = parse_line(&l, attr_delimit_char).unwrap();

            if option.is_empty() {
                continue;
            }

            let opt_props = OptionProperties::new(option, primary_value, attr_vec);
            vec.push(opt_props);

            // Show the line and its number.
            println!("{}. {}", index + 1, l);
        }
        Ok(vec)
    }

    /// Returns the properties of the option, derived from
    /// a line in the configuration file.
    fn parse_line(l: &str, attr_delimit_char: char) -> io::Result<(String, String, Vec<String>)> {
        let line = l.trim();
        if line.is_empty() || line.as_bytes()[0] == b'#' {
            return Ok(("".to_string(), "".to_string(), vec![]));
        }

        let mut i = line.find('=');
        let (option, value) = match i.is_some() {
            true => (
                format!("{}", &line[..i.unwrap()].trim()),
                format!("{}", &line[i.unwrap() + 1..].trim()),
            ),
            false => (line.to_string(), String::new()),
        };

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

        Ok((option, primary_value, attr_vec))
    }

    #[test]
    fn test_parse_line() {
        // Test with no attributes
        assert_eq!(
            parse_line("Option = /home/foo", ',').unwrap(),
            ("Option".to_string(), "/home/foo".to_string(), vec![])
        );

        // Test with 5 attributes and several spaces
        assert_eq!(
            parse_line("Option=/home/foo , another  ,   test,1,2,3", ',').unwrap(),
            (
                "Option".to_string(),
                "/home/foo".to_string(),
                vec![
                    "another".to_string(),
                    "test".to_string(),
                    "1".to_string(),
                    "2".to_string(),
                    "3".to_string()
                ]
            )
        );

        // Test with leading '#' sign
        assert_eq!(
            parse_line("#Option = /home/foo", ',').unwrap(),
            ("".to_string(), "".to_string(), vec![])
        );

        // Test with two attributes, a single space after the commas
        assert_eq!(
            parse_line("Option = /home/foo, removable, test", ',').unwrap(),
            (
                "Option".to_string(),
                "/home/foo".to_string(),
                vec!["removable".to_string(), "test".to_string()]
            )
        );

        // Test for blank line
        assert_eq!(
            parse_line("        ", ',').unwrap(),
            ("".to_string(), "".to_string(), vec![])
        );
    }

    #[test]
    #[ignore]
    fn test_parse_line_FIX_ME() {
        // A case like this needs to be handled. It's an invalid line in the config
        // file. An option can have no value (e.g., DefaultOptionOff), but if there is
        // something after it, it requires an '=' sign.
        assert_eq!(
            parse_line("WASTE  /home/foo", '=').unwrap(),
            ("".to_string(), "".to_string(), vec![])
        );
    }
}
