use iterchunks::IterArrayChunks;
use once_cell::sync::Lazy;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
struct CargoState {
    containers: Vec<Vec<char>>,
}

#[derive(Debug)]
struct CraneMove {
    amount: usize,
    source_stack_id: usize,
    target_stack_id: usize,
}

enum CraneVersion {
    V1,
    V2,
}

#[derive(Debug)]
struct CraneMoveParseError {}
static MOVE_FORMAT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap());

impl FromStr for CraneMove {
    type Err = CraneMoveParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let capture = MOVE_FORMAT_REGEX.captures(s).unwrap();

        let get_match = |i: usize| -> usize { capture.get(i).unwrap().as_str().parse().unwrap() };

        Ok(CraneMove {
            amount: get_match(1),
            source_stack_id: get_match(2),
            target_stack_id: get_match(3),
        })
    }
}

impl CargoState {
    fn get_top_items(&self) -> String {
        let mut result = String::from("");

        for stack in self.containers.iter() {
            if let Some(top) = stack.last() {
                result.push(*top);
            }
        }

        result
    }

    fn apply_moves(&mut self, crane_moves: Vec<CraneMove>, crane_version: CraneVersion) {
        for crane_move in crane_moves {
            match crane_version {
                CraneVersion::V1 => self.apply_move_v1(crane_move),
                CraneVersion::V2 => self.apply_move_v2(crane_move),
            }
        }
    }

    /// In v1 version we pop one-by-one
    /// Example:
    /// Initial state:
    /// ```
    /// [J]
    /// [R] [T]
    /// ```
    /// Command: move 2 from 1 to 2
    /// Result state:
    /// ```
    ///    [R]
    ///    [J]
    ///    [T]
    /// ```
    fn apply_move_v1(&mut self, crane_move: CraneMove) {
        let source_id = crane_move.source_stack_id - 1;
        let target_id = crane_move.target_stack_id - 1;

        for _ in 0..crane_move.amount {
            let item = self.containers[source_id].pop().unwrap();

            self.containers[target_id].push(item);
        }
    }

    /// In v2 version we pop by batch of requested size (keeping batch in the initial order)
    /// Example:
    /// Initial state:
    /// ```
    /// [J]
    /// [R] [T]
    /// ```
    /// Command: move 2 from 1 to 2
    /// Result state:
    /// ```
    ///    [J]
    ///    [R]
    ///    [T]
    /// ```
    fn apply_move_v2(&mut self, crane_move: CraneMove) {
        let source_id = crane_move.source_stack_id - 1;
        let target_id = crane_move.target_stack_id - 1;

        let mut tmp: Vec<char> = Vec::new();

        for _ in 0..crane_move.amount {
            let item = self.containers[source_id].pop().unwrap();

            tmp.push(item);
        }

        for _ in 0..crane_move.amount {
            let item = tmp.pop().unwrap();

            self.containers[target_id].push(item);
        }
    }
}

fn main() {
    solve(CraneVersion::V1);
    solve(CraneVersion::V2);
}

fn solve(version: CraneVersion) {
    let initial_state: Vec<&str> = include_str!("../input.txt")
        .lines()
        .take_while(|s| !s.is_empty())
        .collect();

    let moves: Vec<CraneMove> = include_str!("../input.txt")
        .lines()
        .filter(|s| s.starts_with("move"))
        .map(|s| s.parse::<CraneMove>().unwrap())
        .collect();

    let mut state = parse_initial_state(initial_state);
    println!("Source state: {:?}", state);

    state.apply_moves(moves, version);
    println!("Final state: {:?}", state);

    let top = state.get_top_items();
    println!("Top: {:?}", top);
}

fn parse_initial_state(lines: Vec<&str>) -> CargoState {
    let index_line = lines.last().unwrap();
    let stack_ids: Vec<&str> = index_line.trim().split_ascii_whitespace().collect();

    // Initialize state with n empty stacks
    let mut state = CargoState {
        containers: vec![Vec::new(); stack_ids.len()],
    };

    // Fill stacks from bottom to the top
    let mut i = stack_ids.len() - 1;
    while i > 0 {
        // Align with one trailing space to have integer amount of 4-symbol blocks
        let l = lines[i - 1].to_string() + " ";

        let containers = l
            .chars()
            .array_chunks::<4>()
            .enumerate()
            .filter(|c| c.1[0] == '[');

        for (container_id, item_type) in containers {
            state.containers[container_id].push(item_type[1]);
        }

        i -= 1;
    }

    state
}
