use std::collections::HashSet;
// on version 1.65.0
// test test_multiple_anagrams ... bench:       1,787 ns/iter (+/- 287)
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let word_sorted = get_sorted(&word_lower);
    possible_anagrams
        .iter()
        .filter(|candidate| {
            let candidate_lower = candidate.to_lowercase();
            candidate_lower.len() == word_lower.len()
                && candidate_lower != word_lower
                && get_sorted(&candidate_lower) == word_sorted
        })
        .copied()
        .collect()
}
fn get_sorted(word: &str) -> Vec<char> {
    let mut word_sorted: Vec<char> = word.chars().collect();
    word_sorted.sort_unstable();
    word_sorted
}