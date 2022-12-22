use itertools::Itertools;
const INPUT: &str = include_str!("../inputs/day5.txt");

#[derive(Debug)]
struct Command {
    from: usize,
    to: usize,
    amount: usize
}

pub fn solution_a() -> String {
    let (mut stacks, commands) = parse_input().unwrap();
    for Command{from, to, amount} in commands {
        for _ in 0..amount{
            if let Some(removed) = stacks[from].pop(){
                stacks[to].push(removed);
            }
        }
    }

    stacks
        .iter()
        .filter_map(|stack| stack.iter().last() )
        .collect()
}

pub fn solution_b() -> String {
    let (mut stacks, commands) = parse_input().unwrap();
    for Command{from, to, amount} in commands {
        let len_stack = stacks[from].len();
        let removed = stacks[from].split_off(len_stack - amount);
        stacks[to].extend(removed);
    }

    stacks
        .iter()
        .filter_map(|stack| stack.iter().last() )
        .collect()
}


fn parse_input() -> Option<(Vec<Vec<char>>, Vec<Command>)>{
    let (left, commands_str) = INPUT.split_once("\n\n")?;
    let (stacks_str, platforms) = left.rsplit_once("\n")?;
    let num_stacks: usize = platforms.split_whitespace().last()?.parse().unwrap();
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); num_stacks];
    for line in stacks_str.lines().rev() {
        for (idx, mut chunk) in line.chars().chunks(4).into_iter().enumerate(){
            let second = chunk.nth(1)?;
            if second.is_alphabetic() {
                stacks[idx].push(second);
            }
        }
    }

    let mut commands: Vec<Command> = Vec::new();
    for line in commands_str.lines(){
        let line = line.strip_prefix("move ")?;
        let (amount, line) = line.split_once(" from ")?;
        let (from, to) = line.split_once(" to ")?;
        commands.push(
            Command { 
                from: from.parse::<usize>().unwrap() -1, 
                to: to.parse::<usize>().unwrap() -1, 
                amount: amount.parse::<usize>().unwrap() 
            });
    }

    Some((stacks, commands))
}