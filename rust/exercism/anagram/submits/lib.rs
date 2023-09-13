use std::collections::{HashSet, HashMap};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_m = fq(word);
    let mut output = HashSet::<&str>::new();
    for &s in possible_anagrams {
        let s_m: HashMap<String, i32> = fq(s);
        if word.len() == s.len() && !word.to_lowercase().eq(&s.to_lowercase()) && word_m == s_m {
            output.insert(s);
        }
    }
    output
}

fn fq(word: &str) -> HashMap<String, i32> {
    let mut m: HashMap<String, i32> = HashMap::<String, i32>::new();
    for c in word.chars() {
        m.entry(c.to_lowercase().to_string()).and_modify(|v| *v += 1 ).or_insert(1);
    }
    m
}


/* 
use unicode_segmentation::UnicodeSegmentation;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_m = fq(word);
    let mut output = HashSet::<&str>::new();
    for &s in possible_anagrams {
        let s_m = fq(s);
        if !word.to_ascii_lowercase().eq(&s.to_ascii_lowercase()) && word_m == s_m {
            output.insert(s);
        }
    }
    output
}

fn fq(word: &str) -> HashMap<String, i32> {
    let mut m = HashMap::<String, i32>::new();
    for s in word.graphemes(true) {
        m.entry(s.to_string().to_lowercase()).and_modify(|v| *v += 1 ).or_insert(1);
    }
    m
}
*/