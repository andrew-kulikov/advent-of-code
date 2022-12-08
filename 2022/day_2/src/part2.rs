#[path = "./constants.rs"]
mod constants;

const REQUIRE_LOSE: &str = "X";
const REQUIRE_DRAW: &str = "Y";
const REQUIRE_WIN: &str = "Z";

pub fn solve_part_2() {
    let total: i32 = include_str!("../data/input.txt")
        .lines()
        .map(|l| calculate_round_total(l))
        .sum();

    println!("{:?}", total);
}

fn calculate_round_total(round: &str) -> i32 {
    let mut parts = round.split(" ");

    let opponent = parts.next().unwrap();
    let exprected_result = parts.next().unwrap();

    let turn_score = match (opponent, exprected_result) {
        (constants::OPPONENT_PLAYER_ROCK, REQUIRE_LOSE) => constants::SCISSORS_SCORE,
        (constants::OPPONENT_PLAYER_ROCK, REQUIRE_DRAW) => constants::ROCK_SCORE,
        (constants::OPPONENT_PLAYER_ROCK, REQUIRE_WIN) => constants::PAPER_SCORE,

        (constants::OPPONENT_PLAYER_PAPER, REQUIRE_LOSE) => constants::ROCK_SCORE,
        (constants::OPPONENT_PLAYER_PAPER, REQUIRE_DRAW) => constants::PAPER_SCORE,
        (constants::OPPONENT_PLAYER_PAPER, REQUIRE_WIN) => constants::SCISSORS_SCORE,

        (constants::OPPONENT_PLAYER_SCISSORS, REQUIRE_LOSE) => constants::PAPER_SCORE,
        (constants::OPPONENT_PLAYER_SCISSORS, REQUIRE_DRAW) => constants::SCISSORS_SCORE,
        (constants::OPPONENT_PLAYER_SCISSORS, REQUIRE_WIN) => constants::ROCK_SCORE,

        _ => 0,
    };

    let game_score = match exprected_result {
        REQUIRE_LOSE => constants::LOSS_SCORE,
        REQUIRE_DRAW => constants::DRAW_SCORE,
        REQUIRE_WIN => constants::VICTORY_SCORE,
        _ => 0,
    };

    game_score + turn_score
}
