use crate::constants;

pub fn from_str(s: &str) -> Option<&str> {
    match s {
        "english" => Some(constants::ENGLISH),
        _ => None,
    }
}