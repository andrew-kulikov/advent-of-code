use std::collections::HashMap;

fn part_1() {
    let (left, right): (Vec<i32>, Vec<i32>) = include_str!("../input.txt")
        .lines()
        .map(|line| {
            let mut nums = line.split("   ").map(|num| num.parse::<i32>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .unzip();

    let mut left_sorted = left.clone();
    left_sorted.sort();
    let mut right_sorted = right.clone();
    right_sorted.sort();

    let result: i32 = left_sorted
        .iter()
        .zip(right_sorted.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    print!("Result: {:?}", result);
}

fn part_2() {
    let (left, right): (Vec<i32>, Vec<i32>) = include_str!("../input.txt")
        .lines()
        .map(|line| {
            let mut nums = line.split("   ").map(|num| num.parse::<i32>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .unzip();

    let mut right_counts = HashMap::<i32, i32>::new();
    right.iter().for_each(|r| {
        if let Some(v) = right_counts.get_mut(r) {
            *v += 1;
        } else {
            right_counts.insert(*r, 1);
        }
    });

    let result: i32 = left
        .iter()
        .map(|num| num * match right_counts.get(num) {
            Some(v) => *v,
            None => 0,
        })
        .sum();

    print!("Result: {:?}", result);
}

fn main() {
    part_2();
}
