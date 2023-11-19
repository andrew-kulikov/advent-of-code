fn print_matrix(matrix: &Vec<Vec<u32>>) {
    for row in matrix {
        for &element in row {
            print!("{} ", element);
        }
        println!();
    }
}

fn create_matrix(rows: usize, cols: usize, fill: u32) -> Vec<Vec<u32>> {
    vec![vec![fill; cols]; rows]
}

fn solve_part1(matrix: &Vec<Vec<u32>>) -> u32 {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut left = create_matrix(rows, cols, 0);
    for i in 0..rows {
        let mut cur_max = 0;
        for j in 0..cols {
            if j == 0 || matrix[i][j] > cur_max {
                cur_max = matrix[i][j];
                left[i][j] = 1;
            }
        }
    }
    // println!("Left");
    // print_matrix(&left);

    let mut right = create_matrix(rows, cols, 0);
    for i in 0..rows {
        let mut cur_max = 0;
        for j in (0..cols).rev() {
            if j == cols - 1 || matrix[i][j] > cur_max {
                cur_max = matrix[i][j];
                right[i][j] = 1;
            }
        }
    }
    // println!("Right");
    // print_matrix(&right);

    let mut top = create_matrix(rows, cols, 0);
    for j in 0..cols {
        let mut cur_max = 0;
        for i in 0..rows {
            if i == 0 || matrix[i][j] > cur_max {
                cur_max = matrix[i][j];
                top[i][j] = 1;
            }
        }
    }
    // println!("Top");
    // print_matrix(&top);

    let mut bot = create_matrix(rows, cols, 0);
    for j in 0..cols {
        let mut cur_max = 0;
        for i in (0..rows).rev() {
            if i == rows - 1 || matrix[i][j] > cur_max {
                cur_max = matrix[i][j];
                bot[i][j] = 1;
            }
        }
    }
    // println!("Bot");
    // print_matrix(&bot);

    let mut visible = create_matrix(rows, cols, 0);
    for i in 0..rows {
        for j in 0..cols {
            if left[i][j] == 1 || right[i][j] == 1 || top[i][j] == 1 || bot[i][j] == 1 {
                visible[i][j] = 1;
            }
        }
    }
    // println!("Result");
    // print_matrix(&visible);

    visible.iter().flatten().sum()
}

fn solve_part2(matrix: &Vec<Vec<u32>>) -> u32 {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut left = create_matrix(rows, cols, 0);
    for i in 0..rows {
        for j in 0..cols {
            for k in (0..j).rev() {
                left[i][j] += 1;
                if matrix[i][k] >= matrix[i][j] {
                    break;
                }
            }
        }
    }
    // println!("Left");
    // print_matrix(&left);

    let mut right = create_matrix(rows, cols, 0);
    for i in 0..rows {
        for j in (0..cols).rev() {
            for k in j + 1..cols {
                right[i][j] += 1;
                if matrix[i][k] >= matrix[i][j] {
                    break;
                }
            }
        }
    }
    // println!("Right");
    // print_matrix(&right);

    let mut top = create_matrix(rows, cols, 0);
    for j in 0..cols {
        for i in 0..rows {
            for k in (0..i).rev() {
                top[i][j] += 1;
                if matrix[k][j] >= matrix[i][j] {
                    break;
                }
            }
        }
    }
    // println!("Top");
    // print_matrix(&top);

    let mut bot = create_matrix(rows, cols, 0);
    for j in 0..cols {
        for i in (0..rows).rev() {
            for k in i + 1..rows {
                bot[i][j] += 1;
                if matrix[k][j] >= matrix[i][j] {
                    break;
                }
            }
        }
    }
    // println!("Bot");
    // print_matrix(&bot);

    let mut scenic_scores = create_matrix(rows, cols, 0);
    for i in 0..rows {
        for j in 0..cols {
            scenic_scores[i][j] = left[i][j] * right[i][j] * top[i][j] * bot[i][j];
        }
    }
    // println!("Result");
    // print_matrix(&scenic_scores);

    scenic_scores.iter().flatten().max().unwrap().clone()
}

fn main() {
    let matrix = include_str!("../input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let answer = solve_part2(&matrix);

    print!("{}", answer);
}
