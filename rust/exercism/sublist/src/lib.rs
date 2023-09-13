use std::cmp;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {

    let max_matched = if _first_list.len() < _second_list.len() {
        max_matched(_second_list, _first_list) 
    } else { 
        max_matched(_first_list, _second_list) 
    };

    if _first_list.len() == _second_list.len() && max_matched == _first_list.len() {
        Comparison::Equal
    } else if _first_list.len() < _second_list.len() && max_matched == _first_list.len() {
        Comparison::Sublist
    } else if _first_list.len() > _second_list.len() && max_matched == _second_list.len() {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}

fn max_matched<T: PartialEq>(hey: &[T], needle: &[T]) -> usize {
    let mut max_matched = 0;
    for f in 0..hey.len() {
        let mut matched = 0;
        let mut f_idx = f;
        for s in 0..needle.len() {
            if f_idx >= hey.len() || hey[f_idx] != needle[s] {
                matched = 0;
                break;
            } else {
                f_idx = f_idx + 1; 
                matched += 1;
                max_matched = cmp::max(max_matched, matched);
            }
        } 
    }

    max_matched
}
