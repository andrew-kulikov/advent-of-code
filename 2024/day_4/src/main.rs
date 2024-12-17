const CHARS: [char; 4] = ['X', 'M', 'A', 'S'];

#[derive(Debug, Clone)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct Point2D {
    x: i32,
    y: i32,
}

const DIRECTIONS: [Point2D; 8] = [
    Point2D { x: -1, y: -1 },
    Point2D { x: -1, y: 0 },
    Point2D { x: -1, y: 1 },
    Point2D { x: 0, y: -1 },
    Point2D { x: 0, y: 1 },
    Point2D { x: 1, y: -1 },
    Point2D { x: 1, y: 0 },
    Point2D { x: 1, y: 1 },
];

fn is_is_boundary(lines: &Vec<Vec<char>>, pos: &Pos) -> bool {
    return pos.x >= lines.len() || pos.y >= lines[pos.x].len();
}

fn transform(pos: &Pos, direction: &Point2D) -> Option<Pos> {
    let new_x = pos.x as i32 + direction.x;
    let new_y = pos.y as i32 + direction.y;
    if new_x < 0 || new_y < 0 {
        return None;
    }
    return Some(Pos {
        x: new_x as usize,
        y: new_y as usize,
    });
}

fn is_word(lines: &Vec<Vec<char>>, pos: &Pos, direction: &Point2D) -> bool {
    let mut cur = pos.clone();

    for c in CHARS.iter() {
        if lines[cur.x][cur.y] != *c {
            return false;
        }

        if *c == 'S' {
            break;
        }

        let new_pos = transform(&cur, direction);
        if new_pos.is_none() {
            return false;
        }

        cur = new_pos.unwrap();
        if is_is_boundary(lines, &cur) {
            return false;
        }
        //println!("{} - {:?} - {:?}", lines[cur.x][cur.y], cur, direction);
    }
    return true;
}

fn get_xmas_count(lines: &Vec<Vec<char>>, pos: &Pos) -> usize {
    if lines[pos.x][pos.y] != 'X' {
        return 0;
    }

    return DIRECTIONS
        .iter()
        .filter(|&direction| is_word(lines, pos, direction))
        .count();
}

fn part_1() {
    let lines = include_str!("../input.txt")
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut total_cnt = 0;
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            let pos = Pos { x: i, y: j };
            total_cnt += get_xmas_count(&lines, &pos);
        }
    }

    println!("part1: {:?}", total_cnt);
}

fn part_2() {
    let lines = include_str!("../input.txt")
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut total_cnt = 0;
    for i in 1..lines.len() - 1 {
        for j in 1..lines[i].len() - 1 {
            if lines[i][j] != 'A' {
                continue;
            }
            if (lines[i - 1][j - 1] == 'M' && lines[i + 1][j + 1] == 'S'
                || lines[i - 1][j - 1] == 'S' && lines[i + 1][j + 1] == 'M')
                && (lines[i - 1][j + 1] == 'M' && lines[i + 1][j - 1] == 'S'
                    || lines[i - 1][j + 1] == 'S' && lines[i + 1][j - 1] == 'M')
            {
                total_cnt += 1;
            }
        }
    }

    println!("part2: {:?}", total_cnt);
}

fn main() {
    part_1();
    part_2();
}
