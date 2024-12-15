use regex::Regex;

fn get_product(a: &str, b: &str) -> i64 {
    a.parse::<i64>().unwrap() * b.parse::<i64>().unwrap()
}

fn part_1() {
    let line = include_str!("../input.txt");
    let mul_gerex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let sum = mul_gerex
        .captures_iter(line)
        .map(|c| c.extract())
        .fold(0, |acc, (_, [a, b])| acc + get_product(a, b));

    println!("part1: {:?}", sum);
}

#[derive(Debug)]
struct Command {
    allow_operations: bool,
    start_pos: usize,
}

fn part_2() {
    let line = include_str!("../input.txt");
    let mul_gerex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let control_gerex = Regex::new(r"don't\(\)|do\(\)").unwrap();

    // True if execution enabled, false if disabled
    let mut commands = Vec::<Command>::new();
    for c in control_gerex.captures_iter(line) {
        let _match = c.get(0).unwrap();
        commands.push(Command {
            allow_operations: _match.as_str() == "do()",
            start_pos: _match.start(),
        });
    }

    let mut sum: i64 = 0;
    let mut is_execution_enabled = true;
    let mut cur_command = 0;
    for c in mul_gerex.captures_iter(line) {
        let _match = c.get(0).unwrap();

        if commands.len() > 0 {
            while cur_command < commands.len() - 1
                && commands[cur_command + 1].start_pos < _match.start()
            {
                cur_command += 1;
            }
            is_execution_enabled = commands[cur_command].allow_operations;
        }

        let (_, [a, b]) = c.extract();
        let product = get_product(a, b);

        if is_execution_enabled {
            sum += product;
        }
    }

    println!("part2: {:?}", sum);
}

fn main() {
    part_1();
    part_2();
}
