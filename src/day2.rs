const INPUT: &str = include_str!("../inputs/day2.txt");
#[derive(Debug)]
enum Pick {
    Rock,
    Paper,
    Scissors
}

pub fn solution_a() -> usize{
    let mut score: usize = 0;
    for line in INPUT.lines(){
        let (my_pick,others_pick) = decode_picks(line);
        score = score +
         pick_value(&my_pick) + 
        match_value(&my_pick, &others_pick)
    }
    score
}

pub fn solution_b() -> usize{
    let mut score: usize = 0;
    for line in INPUT.lines(){
        let (_,others_pick) = decode_picks(&line);
        let my_pick = what_to_pick(&others_pick, &line[2..3]);
        score = score +
         pick_value(&my_pick) + 
        match_value(&my_pick, &others_pick)
    }
    score
}

fn what_to_pick(others_pick : &Pick, outcome: &str) -> Pick {
    match outcome {
        //Lose
        "X" => {
            match others_pick {
                Pick::Rock => Pick::Scissors,
                Pick::Paper => Pick::Rock,
                Pick::Scissors => Pick::Paper
            }
        },
        //Draw
        "Y" => {
            match others_pick {
                Pick::Rock => Pick::Rock,
                Pick::Paper => Pick::Paper,
                Pick::Scissors => Pick::Scissors
            }
        },
        //Win
        "Z"=> {
            match others_pick {
                Pick::Rock => Pick::Paper,
                Pick::Paper => Pick::Scissors,
                Pick::Scissors => Pick::Rock
            }

        },
        _ => std::process::exit(0)
    }
}

fn decode_picks(line: &str) -> (Pick, Pick){
    let yours = match &line[0..1] {
        "A" => Pick::Rock,
        "B" => Pick::Paper,
        "C" => Pick::Scissors,
        _ => std::process::exit(0)
    };
    let mine = match &line[2..3] {
        "X" => Pick::Rock,
        "Y" => Pick::Paper,
        "Z" => Pick::Scissors,
        _ => std::process::exit(0)
    };
    (mine,yours)
}

fn pick_value(pick: &Pick) -> usize {
    match pick {
        Pick::Rock => 1,
        Pick::Paper => 2,
        Pick::Scissors => 3
    }
}

fn match_value(my_pick: &Pick, others_pick: &Pick) -> usize {
    match my_pick {
        Pick::Rock => {
            match others_pick {
                Pick::Rock => 3,
                Pick::Paper => 0,
                Pick::Scissors => 6
            }
        },
        Pick::Paper => {
            match others_pick {
                Pick::Rock => 6,
                Pick::Paper => 3,
                Pick::Scissors => 0
            }
        },
        Pick::Scissors => {
            match others_pick {
                Pick::Rock => 0,
                Pick::Paper => 6,
                Pick::Scissors => 3
            }

        }
    }
}