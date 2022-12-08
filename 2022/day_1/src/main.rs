use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./data/input.txt") {
        let batches = read_buckets(lines);
        let mut elves_totals = calculate_elves_totals(batches);

        elves_totals.sort();

        let max_value = elves_totals.last().unwrap();
        let best_three_elves = elves_totals.iter().rev().take(3).collect::<Vec<&i32>>();
        let best_three_elves_sum = best_three_elves.iter().map(|&x| x).sum::<i32>();

        println!("{}", *max_value);
        println!("{:?}, sum: {}", best_three_elves, best_three_elves_sum);
    }
}

fn calculate_elves_totals(batches: Vec<Vec<i32>>) -> Vec<i32> {
    batches
        .into_iter()
        .map(|numbers| numbers.into_iter().sum::<i32>())
        .collect()
}

fn read_buckets(lines: io::Lines<io::BufReader<File>>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut cur: Vec<i32> = Vec::new();

    for line in lines {
        if let Ok(s) = line {
            if !s.is_empty() {
                let num = s.parse::<i32>().unwrap();
                cur.push(num);
            } else {
                result.push(cur.clone());
                cur.clear();
            }
        }
    }

    result.push(cur.clone());
    cur.clear();

    result
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf_reader = io::BufReader::new(file);

    Ok(buf_reader.lines())
}
