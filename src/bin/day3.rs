use std::collections::HashSet;


fn priority(c: char) -> i32 {
    1 + match c {
        'a'..='z' => c as i32 - 'a' as i32,
        'A'..='Z' => 26 + c as i32 - 'A' as i32,
        _ => unreachable!(),
    }
}

fn split_half(s: &str) -> (&str, &str) {
    if s.len() % 2 == 1 {
        panic!("odd length string");
    }
    let mid = s.len() / 2;
    s.split_at(mid)
}

fn get_repeats(s: &str) -> Vec<char> {
    let mut repeats = HashSet::new();
    let (left, right) = split_half(s);

    // hash set of characters in the left half
    let mut left_chars = HashSet::new();
    for c in left.chars() {
        left_chars.insert(c);
    }

    // check if any characters in the right half are in the left half
    for c in right.chars() {
        if left_chars.contains(&c) {
            repeats.insert(c);
        }
    }

    repeats.into_iter().collect()
}

fn part_1(input: &str) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        let repeats = get_repeats(line);
        for c in repeats {
            sum += priority(c);
        }
    }
    sum
}

fn chunks_of_3(input: &str) -> Vec<Vec<&str>> {
    let mut chunks = Vec::new();
    let mut chunk = Vec::new();
    for line in input.lines() {
        chunk.push(line);
        if chunk.len() == 3 {
            chunks.push(chunk);
            chunk = Vec::new();
        }
    }
    if chunk.len() != 0 {
        panic!("wrong number of lines");
    }
    chunks
}

fn get_badge(group: Vec<&str>) -> char {
    // find the character that appears in all three lines
    let mut chars = HashSet::new();
    for c in group[0].chars() {
        chars.insert(c);
    }
    for c in group[1].chars() {
        chars.insert(c);
    }
    for c in group[2].chars() {
        chars.insert(c);
    }

    for c in chars {
        if group[0].contains(c) && group[1].contains(c) && group[2].contains(c) {
            return c;
        }
    }

    panic!("no character appears in all three lines");
}

fn part_2(input: &str) -> i32 {
    let mut sum = 0;
    for group in chunks_of_3(input) {
        let badge = get_badge(group);
        sum += priority(badge);
    }
    sum
}

fn main() {
    let input = include_str!("../../inputs/3.txt");
    println!("part 1: {}", part_1(input));
    println!("part 2: {}", part_2(input));
}

#[cfg(test)]
mod tests {
    use std::panic::PanicInfo;

    use super::*;

    const INPUT: &str = 
"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_priority() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('z'), 26);
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('Z'), 52);
    }

    #[test]
    fn test_split_half() {
        assert_eq!(split_half("abcd"), ("ab", "cd"));        
    }

    #[test]
    #[should_panic(expected = "odd length string")]
    fn test_split_half_odd() {
        let f  = |_: &PanicInfo| {};
        std::panic::set_hook(Box::new(f));
        split_half("abcde");
    }

    #[test]
    fn test_repeats() {
        assert_eq!(get_repeats("abca"), vec!['a']);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 157);
    }

    #[test]
    fn test_chunking() {
        let chunks = chunks_of_3(INPUT);
        assert_eq!(chunks, vec![
            vec!["vJrwpWtwJgWrhcsFMMfFFhFp", "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", "PmmdzqPrVvPwwTWBwg"],
            vec!["wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", "ttgJtRGJQctTZtZT", "CrZsJsPPZsGzwwsLwLmpwMDw"],
        ]);
    }

    #[test]
    fn test_get_badge() {
        assert_eq!(get_badge(vec!["abc", "adb", "abe"]), 'b');
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 70);
    }
}