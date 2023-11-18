use std::collections::HashSet;

pub fn solve() {
    let total: i32 = include_str!("../input.txt")
        .lines()
        .map(|l| find_duplicated_item(l))
        .map(|i| get_item_weight(i) as i32)
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

fn find_duplicated_item(items: &str) -> char {
    let (half1, half2) = items.split_at(items.len() / 2);

    let mut unique_items = HashSet::new();

    for item in half1.chars() {
        unique_items.insert(item);
    }

    // We know that exactly one item is duplicate
    // So case when no duplicated found is no possible in this task and we panic if no duplicates found
    for item in half2.chars() {
        if unique_items.contains(&item) {
            println!("{:?} has duplicated item {:?}", items, item);
            return item;
        }
    }

    panic!("no duplicated items found")
}
