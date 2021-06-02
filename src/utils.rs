use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct PercentDecodeError(&'static str);

impl fmt::Display for PercentDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn percent_encode(to_encode: &str) -> String {
    to_encode.chars()
        .map(|c| percent_encode_reserved(c))
        .fold(String::new(), |acc, s| acc + s.as_str())
}

pub fn percent_decode(to_decode: &str) -> Result<String, PercentDecodeError> {
    let mut chars = to_decode.chars();
    let mut result = String::new();
    while let Some(c) = chars.next() {
        if c == '%' {
            let mut char_code = String::new();
            char_code.push(chars.next().ok_or(PercentDecodeError("Char missing in encoding."))?);
            char_code.push(chars.next().ok_or(PercentDecodeError("Char missing in encoding."))?);
            result.push(u8::from_str_radix(&char_code, 16).map_err(|_| PercentDecodeError("Error parsing hex to char."))? as char);
        }
        else {
            result.push(c);
        }
    }
    Ok(result)
}

pub fn percent_encode_reserved(to_encode: char) -> String {
    if is_unreserved(to_encode) {
        to_encode.to_string()
    }
    else {
        percent_encode_char(to_encode)
    }
}

pub fn percent_encode_char(to_encode: char) -> String {
    format!("%{:02x}", to_encode as u32)
}

pub fn is_unreserved(check_reserved: char) -> bool {
    if check_reserved.is_ascii() {
        match check_reserved {
            'A'..='Z' | 'a'..='z' | '0'..='9' => true,
            '-'| '_' | '~' | '.' | '/' => true,
            _ => false
        }
    }
    else {
        true
    }
}

/// See the original [source here](https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html).
/// 
/// The output is wrapped in a Result to allow matching on errors
/// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[test]
fn test_escape_url() {
    assert_eq!(percent_encode("~/foo/bar&/baz boom"),"~/foo/bar%26/baz%20boom");
    assert_eq!(percent_encode("~/hello&there/🐢👀🍻"), "~/hello%26there/🐢👀🍻");
}

#[test]
fn test_unescape_url() {
    assert_eq!(percent_decode("~/foo/bar%26/baz%20boom").unwrap(), "~/foo/bar&/baz boom");
    assert_eq!(percent_decode("~/hello%26there/🐢👀🍻").unwrap(),"~/hello&there/🐢👀🍻");
}

#[test]
fn test_invalid_unescape() {
    assert_eq!(percent_decode("~/foo/bar%26/baz%20boom%"), Err(PercentDecodeError("Char missing in encoding.")));
    assert_eq!(percent_decode("~/foo/bar%26/baz%2Zboom%"), Err(PercentDecodeError("Error parsing hex to char.")));
}
