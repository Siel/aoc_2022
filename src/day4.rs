use regex::Regex;
const INPUT: &str = include_str!("../inputs/day4.txt");

pub fn solution_a() -> usize {
    let mut score: usize = 0;
    for line in INPUT.lines(){
        let (one,other) = parse_line(line);
        if one.contains(&other) || other.contains(&one) {
            score +=1;
        }
    }
    score
}

pub fn solution_b() -> usize {
    let mut score: usize = 0;
    for line in INPUT.lines(){
        let (one,other) = parse_line(line);
        if one.contains(&other) || other.contains(&one) || 
            one.overlaps(&other) || other.overlaps(&one){
            score +=1;
        }
    }
    score
}

#[derive(Debug)]
pub struct Range {
    lower: usize,
    upper: usize
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.lower <= other.lower && self.upper >= other.upper
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.lower <= other.lower && self.upper >= other.lower
    }
}

//instead of using regex, would be faster to split by , an then by -
fn parse_line(line: &str) -> (Range, Range){
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let cap = re.captures(line).unwrap();
    (Range { lower: cap[1].parse::<usize>().unwrap(), upper: cap[2].parse::<usize>().unwrap()},
    Range { lower: cap[3].parse::<usize>().unwrap(), upper: cap[4].parse::<usize>().unwrap()})
}
