use clap_num::number_range;
use unicode_segmentation::UnicodeSegmentation;

/// Shorten a unicode string and add a trail at the end (ex. ...).
/// Can handle Chinese script, emojis, and other scripts.
/// Args:
///     string - the string to shorten
///     len - The length to shorten the description to
///     trail - What to add at the end of the description (ex. ...)
pub fn shorten_unicode(string: String, length: usize, trail: &str) -> String {
    let graphemes = string.grapheme_indices(true);
    let graph_vec: Vec<_> = graphemes.take(length).into_iter().collect();
    let mut unpacked: Vec<&str> = vec![];
    for grapheme in graph_vec {
        unpacked.push(grapheme.1)
    }
    let joined = unpacked.join("");
    
    //dbg!{&graphemes};
    let shortened = format!("{:?}{}", joined, trail);
    shortened
}

pub fn check_limit(s: &str) -> Result<u8, String> {
    number_range(s, 0, 100)
}
