use clap_num::number_range;
use unicode_segmentation::UnicodeSegmentation;

pub fn shorten(string: String, length: usize, trail: &str) -> String {
    let graphemes = string.grapheme_indices(true);
    // let trimmed = &string[..length];
    let graph_vec: Vec<_> = graphemes.take(length).into_iter().collect();
    // let trimmed = graph_vec[..length].join("-");
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
