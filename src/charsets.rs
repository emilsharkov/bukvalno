use crate::constants;

pub fn get_charset_from_label(s: &str) -> Option<&str> {
    match s {
        "english" => Some(constants::ENGLISH),
        "chinese" => Some(constants::CHINESE),
        "japanese" => Some(constants::JAPANESE),
        "braille" => Some(constants::BRAILLE),
        "circles" => Some(constants::CIRCLES),
        "blocks" => Some(constants::BLOCKS),
        _ => None,
    }
}
