use std::ops::RangeInclusive;

fn is_fully_overlapping(a: RangeInclusive<i32>, b: RangeInclusive<i32>) -> bool {
    a.start() <= b.start() && a.end() >= b.end() || b.start() <= a.start() && b.end() >= a.end()
}

fn is_partially_overlapping(a: RangeInclusive<i32>, b: RangeInclusive<i32>) -> bool {
    a.start() <= b.start() && a.end() >= b.start() || b.start() <= a.start() && b.end() >= a.start()
}

fn parse_line(line: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let (left, right) = line.split_once(',').unwrap();
    let (left_x, left_y) = left.split_once('-').unwrap();
    let (right_x, right_y) = right.split_once('-').unwrap();

    (
        left_x.parse().unwrap()..=left_y.parse().unwrap(),
        right_x.parse().unwrap()..=right_y.parse().unwrap(),
    )
}

fn count(input: &str, func: impl Fn(RangeInclusive<i32>, RangeInclusive<i32>) -> bool) -> i32 {
    let mut count = 0;
    for line in input.lines() {
        let (left, right) = parse_line(line);
        if func(left, right) {
            count += 1;
        }
    }
    count
}

fn main() {
    let input = include_str!("../../inputs/4.txt");
    println!("part 1: {}", count(input, is_fully_overlapping));
    println!("part 2: {}", count(input, is_partially_overlapping));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("1-3,2-4"),
            (1..=3, 2..=4)
        );
    }

    #[test]
    fn test_is_fully_overlapping() {
        assert!(is_fully_overlapping(1..=3, 2..=3));
        assert!(is_fully_overlapping(2..=3, 1..=3));
        assert!(!is_fully_overlapping(2..=4, 1..=3));
    }

    #[test]
    fn test_part_1() {
        assert_eq!(count(INPUT, is_fully_overlapping), 2);
        assert_eq!(count(INPUT, is_partially_overlapping), 4);
    }
}