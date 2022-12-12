use std::{collections::HashSet, hash::Hash};

use iterwindows::IterArrayWindows;

//const BATCH_SIZE: usize = 4;
const BATCH_SIZE: usize = 14;

fn main() {
    let result: Vec<usize> = include_str!("../data/input.txt")
        .lines()
        .map(|l| solve(l))
        .collect();

    println!("{:?}", result);
}

fn solve(s: &str) -> usize {
    for (i, window) in s.chars().array_windows::<BATCH_SIZE>().enumerate() {
        if contains_diplicates(window.iter()) {
            return i + BATCH_SIZE;
        }
    }

    0
}

fn contains_diplicates<T>(arr: impl Iterator<Item = T>) -> bool
where
    T: Eq,
    T: Hash,
{
    let mut set: HashSet<T> = HashSet::new();

    for item in arr {
        if !set.insert(item) {
            return false;
        }
    }

    true
}
