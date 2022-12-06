use std::collections::HashSet;

fn search_packet_start(input: &str, marker_length: usize) -> usize {

    for (i, buf) in input.as_bytes().windows(marker_length).enumerate() {
        // check that all bytes are different
        let set: HashSet<&u8> = buf.iter().collect();
        let unique = set.len() == marker_length;
        
        if unique {
            return i + marker_length;
        }
    }

    unreachable!("No packet start found");
}

fn main() {
    let input = include_str!("../../inputs/6.txt");
    println!("Part 1: {}", search_packet_start(input, 4));
    println!("Part 2: {}", search_packet_start(input, 14));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_packet_start() {
        assert_eq!(search_packet_start("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(search_packet_start("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(search_packet_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(search_packet_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }
}