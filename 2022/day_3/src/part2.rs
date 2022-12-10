use iterchunks::IterArrayChunks;
use std::collections::{HashMap, HashSet};

pub fn solve() {
    let total: i32 = include_str!("../data/input.txt")
        .lines()
        .array_chunks::<3>()
        .map(|batch| {
            let common_item = find_common_item(Vec::from(batch));
            println!("{:?} have common item {:?}", batch, common_item);
            common_item
        })
        .map(|item| get_item_weight(item) as i32)
        .sum();

    println!("Total: {:?}", total);
}

// Lowercase item types a through z have priorities 1 through 26.
// Uppercase item types A through Z have priorities 27 through 52.
fn get_item_weight(item: char) -> u8 {
    let char_code = item as u8;

    if item >= 'a' && item <= 'z' {
        char_code - ('a' as u8) + 1
    } else if item >= 'A' && item <= 'Z' {
        char_code - ('A' as u8) + 27
    } else {
        panic!("invalid character")
    }
}

fn find_common_item(batch: Vec<&str>) -> char {
    let batch_len = batch.len();

    let mut item_counts = HashMap::new();

    for items in batch {
        let unique_chars = get_unique_chars(items);

        for item in unique_chars {
            item_counts.entry(item).and_modify(|v| *v += 1).or_insert(1);
        }
    }

    for (item, count) in item_counts {
        if count == batch_len {
            return item;
        }
    }

    panic!("no common items found")
}

fn get_unique_chars(s: &str) -> HashSet<char> {
    let mut result: HashSet<char> = HashSet::new();

    for c in s.chars() {
        result.insert(c);
    }

    result
}
