const INPUT: &str = include_str!("../inputs/day3.txt");

pub fn solution_a() -> usize {
    let mut score: usize = 0;
    for line in INPUT.lines(){
        let (lh,uh) = split_string_by_half(line);
        score = score + find_match_value(encode(lh), encode(uh));
    }
    score
}

pub fn solution_b() -> usize {
    let mut score: usize = 0;
    let mut triplet:[Vec<usize>;3] = Default::default();
    for (i,line) in INPUT.lines().enumerate(){
        let ind = i%3;
        triplet[ind] = encode(line);
        if ind == 2 {
            score = score + find_match_value_3l(&mut triplet);
        }
    }
    score
}


fn split_string_by_half(original: &str) -> (&str, &str){
   let half: usize = original.chars().count()/2;
   //I'm going to ASSUME that size is always even
   (&original[0..half], &original[half..])
}

fn encode(str1: &str) -> Vec<usize>{
    let mut encoded = Vec::with_capacity(str1.chars().count());
    for c in str1.chars(){
        let value = c as usize;
        match value {
            65..=90 =>  encoded.push(value - 38),
            97..=122 =>  encoded.push(value - 96),
            _ => std::process::exit(0)
        }
    }
    encoded
}

fn find_match_value_3l(triplet: &mut [Vec<usize>;3])-> usize {
    let mut vec1 = triplet[0].to_vec();
    let mut vec2 = triplet[1].to_vec();
    let mut vec3 = triplet[2].to_vec();
    vec1.sort_unstable();
    vec2.sort_unstable();
    vec3.sort_unstable();
    for e3 in &vec3{
        for e2 in &vec2{
            if e3 == e2 {
                for e1 in &vec1{
                    if e2 == e1 {
                        return *e1 
                    } else if e1 > e2 {
                        break;
                    }
                }
            } else if e2 > e3 {
                break;
            }
        }
    }
    //No match found
    std::process::exit(0);
}

fn find_match_value(mut vec1: Vec<usize>,mut vec2: Vec<usize>)-> usize {
    //sort the vectors
    vec1.sort_unstable();
    vec2.sort_unstable();

    for e2 in &vec2{
        for e1 in &vec1{
            if e1 == e2 {
                return *e1 
            } else if e1 > e2 {
                break;
            }
        }
    }
    //No match found
    std::process::exit(0);
}



