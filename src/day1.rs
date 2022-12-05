const INPUT: &str = include_str!("../inputs/day1.txt");

pub fn solution_1A() -> usize {

    let mut max_cal: usize = 0;
    let mut acc: usize = 0;
    
    for line in INPUT.lines(){
        match line {
            "" => {
                max_cal = if acc>max_cal {acc}else{max_cal};
                acc =0;
            },
            cal => {
                let cal =match cal.parse::<usize>() {
                    Ok(n) => n,
                    Err(_) => 0
                };
                acc += cal;
            }
            
        }
    }
    max_cal
} 
