pub fn escape_url(to_encode: &String) -> String {
    to_encode.chars()
        .map(|c| percent_encode_reserved(c))
        .fold(String::new(), |acc, s| acc + s.as_str())
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