const INPUT: &str = include_str!("../inputs/day1.txt");

pub fn solution_a() -> usize {

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

pub fn solution_b() -> usize {
    let mut max_cal: usize = 0;
    let mut acc: usize = 0;
    let mut max_3: [usize;3]= [0;3];

    for line in INPUT.lines(){
        match line {
            "" => {
                if acc>max_cal {
                    max_cal = acc
                }
                max_3 = *keep_three(&mut max_3, acc);
                acc = 0;
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
    max_3.iter().sum()
}

fn keep_three(array: &mut [usize;3], val: usize) -> &[usize;3] {
    if array[2]<val {array[2] = val;}
    array.sort_by(|a, b| b.cmp(a));
    array
}
