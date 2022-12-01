fn main() {
    let input = include_str!("../../inputs/1.txt");

    let mut groups:Vec<u32> = input.split("\n\n").map(|group| {
        group
            .lines()
            .map(|line| line.parse::<u32>().unwrap_or(0))
            .sum::<u32>()
    }).collect();

    groups.sort();

    println!("The fattest elf has {} calories", groups.last().unwrap());
    println!("The three fattest elves have {} calories", groups.iter().rev().take(3).sum::<u32>());
}
