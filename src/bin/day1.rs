struct Elf {
    calories: Vec<u32>,
}

impl Elf {
    fn total_calories(&self) -> u32 {
        self.calories.iter().sum()
    }
}

fn main() {
    let input = include_str!("../../inputs/1.txt");

    part_1(input);
    part_2(input);
}

fn fattest_elves(input: &str) -> Vec<Elf> {
    let mut elves = Vec::new();

    // split the input by empty lines
    for group in input.split("\n\n") {
        let mut calories =Vec::new();

        // split the group by lines
        for line in group.lines() {
            match line.parse::<u32>() {
                Ok(calorie) => calories.push(calorie),
                Err(_) => break,
            }
        }

        elves.push(Elf { calories });
    }

    // sort elves by total calories
    elves.sort_by(|a, b| b.total_calories().cmp(&a.total_calories()));

    elves
}

fn part_1(input: &str) {
    let elves = fattest_elves(input);

    // get the elf with the most calories
    if let Some(best) = elves.first() {
        println!("The fattest elf has {} calories", best.total_calories());
    }
}

fn part_2(input: &str) {
    let calories = fattest_elves(input)
        .iter()
        .take(3)
        .map(|e| e.total_calories())
        .sum::<u32>();

    // get the elf with the most calories
    println!("The three fattest elves have {} calories", calories);
}