use clap_num::number_range;

pub fn shorten(string: String, length: usize, trail: &str) -> String {
    let mut truncated = string.clone();
    truncated.truncate(length);
    let shortened = format!("{}{}", truncated, trail);
    shortened
}

pub fn check_limit(s: &str) -> Result<u8, String> {
    number_range(s, 0, 100)
}
