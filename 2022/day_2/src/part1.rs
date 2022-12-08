#[path = "./constants.rs"]
mod constants;

const PLAYER_ROCK: &str = "X";
const PLAYER_PAPER: &str = "Y";
const PLAYER_SCISSORS: &str = "Z";

pub fn solve_part_1() {
    let total: i32 = include_str!("../data/input.txt")
        .lines()
        .map(|l| calculate_round_total(l))
        .sum();

    println!("{:?}", total);
}

fn calculate_round_total(round: &str) -> i32 {
    let mut parts = round.split(" ");

    let opponent = parts.next().unwrap();
    let player = parts.next().unwrap();

    let game_score = match (opponent, player) {
        (constants::OPPONENT_PLAYER_ROCK, PLAYER_ROCK) => constants::DRAW_SCORE,
        (constants::OPPONENT_PLAYER_ROCK, PLAYER_PAPER) => constants::VICTORY_SCORE,
        (constants::OPPONENT_PLAYER_ROCK, PLAYER_SCISSORS) => constants::LOSS_SCORE,

        (constants::OPPONENT_PLAYER_PAPER, PLAYER_ROCK) => constants::LOSS_SCORE,
        (constants::OPPONENT_PLAYER_PAPER, PLAYER_PAPER) => constants::DRAW_SCORE,
        (constants::OPPONENT_PLAYER_PAPER, PLAYER_SCISSORS) => constants::VICTORY_SCORE,

        (constants::OPPONENT_PLAYER_SCISSORS, PLAYER_ROCK) => constants::VICTORY_SCORE,
        (constants::OPPONENT_PLAYER_SCISSORS, PLAYER_PAPER) => constants::LOSS_SCORE,
        (constants::OPPONENT_PLAYER_SCISSORS, PLAYER_SCISSORS) => constants::DRAW_SCORE,

        _ => 0,
    };

    let turn_score = match player {
        PLAYER_ROCK => constants::ROCK_SCORE,
        PLAYER_PAPER => constants::PAPER_SCORE,
        PLAYER_SCISSORS => constants::SCISSORS_SCORE,
        _ => 0,
    };

    game_score + turn_score
}
