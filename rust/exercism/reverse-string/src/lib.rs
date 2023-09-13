use unicode_segmentation::UnicodeSegmentation;

pub fn reverse_iter(input: &str) -> String {
    let mut result = String::new();
    for s in input.graphemes(true) {
        result.insert_str(0, s);
    }
    result
}

pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}