fn parse_line(line: &str) -> Vec<i32> {
    return line
        .split(" ")
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
}

fn is_error(prev: i32, next: i32, increasing: bool) -> bool {
    let diff = next - prev;
    return increasing && (diff < 1 || diff > 3) || !increasing && (diff < -3 || diff > -1);
}

fn is_nums_sequence_safe(nums: &Vec<i32>, skip_index: Option<usize>) -> Result<bool, i32> {
    let skip_id = skip_index.unwrap_or(nums.len());

    let mut i = 1;
    let mut j = 0;

    if i == skip_id {
        i += 1;
    } else if j == skip_id {
        j += 1;
        i += 1;
    }

    let increasing = nums[i] > nums[j];

    while i < nums.len() {
        if is_error(nums[j], nums[i], increasing) {
            return Err(i as i32);
        }
        j = i;
        i += 1;
        if i == skip_id {
            i += 1;
        }
    }

    return Ok(true);
}

fn is_line_safe(line: &str) -> bool {
    let nums = parse_line(line);

    return match is_nums_sequence_safe(&nums, None) {
        Ok(_) => true,
        Err(_) => false,
    };
}

fn part_1() {
    let safe_lines_count = include_str!("../input.txt")
        .lines()
        .filter(|&line| is_line_safe(line))
        .count();

    println!("part1: {:?}", safe_lines_count);
}

fn is_line_safe_or_with_one_error(line: &str) -> bool {
    let nums = line
        .split(" ")
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    return is_nums_sequence_safe(&nums, None)
        .or_else(|err_pos| {
            is_nums_sequence_safe(&nums, Some(err_pos as usize))
                .or_else(|_| is_nums_sequence_safe(&nums, Some(err_pos as usize - 1)))
                .or_else(|_| match err_pos {
                    // Try to skip the first element only if error is between element 1 and 2
                    2 => is_nums_sequence_safe(&nums, Some(err_pos as usize - 2)),
                    _ => Err(err_pos),
                })
        })
        .unwrap_or(false);
}

fn is_line_safe_or_with_one_error_greedy(line: &str) -> bool {
    let nums = parse_line(line);

    for i in 0..nums.len() {
        let mut nums_copy = nums.clone();
        nums_copy.remove(i);
        if is_nums_sequence_safe(&nums_copy, None).is_ok() {
            return true;
        }
    }
    return false;
}

fn part_2() {
    let safe_lines_count = include_str!("../input.txt")
        .lines()
        .filter(|&line| is_line_safe_or_with_one_error(line))
        .count();

    println!("part2: {:?}", safe_lines_count);
}

fn main() {
    part_1();
    part_2();
}
