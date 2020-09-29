pub fn escape_url(to_encode: &str) -> String {
    to_encode.chars()
        .map(|c| percent_encode_reserved(c))
        .fold(String::new(), |acc, s| acc + s.as_str())
}

pub fn unescape_url(to_decode: &str) -> String {
    let mut chars = to_decode.chars();
    let mut result = String::new();
    while let Some(c) = chars.next() {
        if c == '%' {
            let mut char_code = String::new();
            char_code.push(chars.next().expect("char missing in encoding"));
            char_code.push(chars.next().expect("char missing in encoding"));
            result.push(u8::from_str_radix(&char_code, 16).expect("error parsing char code to char") as char);
        }
        else {
            result.push(c);
        }
    }
    result
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