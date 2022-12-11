#[derive(Debug)]
struct PairAssignment {
    first: ElfAssignment,
    second: ElfAssignment,
}

#[derive(Debug)]
struct ElfAssignment {
    start: i32,
    end: i32,
}

impl ElfAssignment {
    fn is_contain(&self, other: &ElfAssignment) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn is_overlap_with(&self, other: &ElfAssignment) -> bool {
        self.start <= other.end && self.end >= other.start
    }
}

impl PairAssignment {
    /// Shows is one of intervals fully covers another
    fn is_one_contains_another(&self) -> bool {
        self.first.is_contain(&self.second) || self.second.is_contain(&self.first)
    }

    /// Shows is one of intervals overlaps wit hanother
    fn is_overlap(&self) -> bool {
        self.first.is_overlap_with(&self.second)
    }
}

fn main() {
    // In part1 we need to calculate amount of pairs where one of elves fully covers other's range
    let full_cover_count = include_str!("../data/input.txt")
        .lines()
        .map(|l| parse_pair_assignment(l))
        .filter(|assignment| {
            let result = assignment.is_one_contains_another();

            println!("{:?} Full cover: {:?}", assignment, result);

            result
        })
        .count();
    
    // In part2 we need to calculate amount of pairs where elves ranges overlaps
    let overlap_count = include_str!("../data/input.txt")
        .lines()
        .map(|l| parse_pair_assignment(l))
        .filter(|assignment| {
            let result = assignment.is_overlap();

            println!("{:?} Overlap: {:?}", assignment, result);

            result
        })
        .count();

    println!("Fully covered: {:?}", full_cover_count);
    println!("Overlaps: {:?}", overlap_count);
}

fn parse_pair_assignment(s: &str) -> PairAssignment {
    let parts: Vec<&str> = s.split(",").collect();

    PairAssignment {
        first: parse_elf_assignment(parts[0]),
        second: parse_elf_assignment(parts[1]),
    }
}

fn parse_elf_assignment(s: &str) -> ElfAssignment {
    let parts: Vec<&str> = s.split("-").collect();

    ElfAssignment {
        start: parts[0].parse().unwrap(),
        end: parts[1].parse().unwrap(),
    }
}
