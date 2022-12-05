use std::collections::HashMap;

type Stacks = HashMap<u8, Vec<String>>;

fn parse_stacks(input: &str) -> Stacks {
    let last_line = input.lines().last().unwrap();
    let input = &input[..input.len() - last_line.len() - 1];

    let stack_names: Vec<u8> = last_line.split_whitespace().map(|s| s.parse::<u8>().unwrap()).collect();
    
    let mut stacks = HashMap::new();

    for line in input.lines() {
        //split line every 4 characters
        let chunks: Vec<String> = line
            .chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .map(|chunk| chunk.iter().collect::<String>().trim().to_owned())
            .collect();

        for (i, chunk) in chunks.iter().enumerate() {
            let stack_name = stack_names[i];
            let stack = stacks.entry(stack_name).or_insert(vec![]);
            if chunk.len() > 0 {
                stack.insert(0, chunk.to_owned());
            }
        }
    }

    stacks

}

struct Op {
    num: i32,
    from: u8,
    to: u8,
}

fn parse_operations(input: &str) -> Vec<Op> {
    let mut ops = vec![];

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        parts.next(); // skip "move"
        let num = parts.next().unwrap().parse::<i32>().unwrap();
        parts.next(); // skip "from"
        let from = parts.next().unwrap().parse().unwrap();
        parts.next(); // skip "to"
        let to = parts.next().unwrap().parse().unwrap();
        ops.push(Op { num, from, to });
    }
    ops
}

fn apply_operations(mut stacks: Stacks, operations: Vec<Op>) -> Stacks {
    for op in operations {
        for _ in 0..op.num {
            let obj = stacks.get_mut(&op.from).unwrap().pop().unwrap();
            stacks.get_mut(&op.to).unwrap().push(obj);
        }
    }
    stacks
}

fn apply_operations_9001(mut stacks: Stacks, operations: Vec<Op>) -> Stacks {
    for op in operations {
        let from_stack = stacks.get_mut(&op.from).unwrap();
        let mut objs = from_stack.split_off(from_stack.len() - op.num as usize);
        stacks.get_mut(&op.to).unwrap().append(&mut objs);
    }
    stacks
}

fn get_tops(input: &str, func: impl Fn(Stacks, Vec<Op>) -> Stacks) -> String {
    let (stacks, operations) = input.split_once("\n\n").unwrap();
    let stacks = parse_stacks(stacks);
    let ops  = parse_operations(operations);
    let stacks = func(stacks, ops);
    
    let mut stacks = stacks.into_iter().collect::<Vec<_>>();
    stacks.sort_by_key(|(k, _)| *k);

    stacks
        .iter()
        .map(|(_, v)| v.last().unwrap())
        .map(|s| s.chars().nth(1).unwrap())
        .fold(String::new(), |a, b| a.to_string() + b.to_string().as_str())
}

fn main() {
    let input = include_str!("../../inputs/5.txt");
    println!("Part 1: {}", get_tops(input, apply_operations));
    println!("Part 2: {}", get_tops(input, apply_operations_9001));
}
