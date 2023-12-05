fn parse_line(line: &str) -> (Vec<u32>, Vec<u32>) {
    let split_line: Vec<Vec<&str>> = line.split(": ").map(|slice| slice.split(" | ").collect()).collect(); 
    let [winning_nums, my_nums] = split_line[1][0..2] else {panic!("unexpected format in line {}", line)};
    let [winning_nums, my_nums]: [Vec<u32>; 2] = [winning_nums, my_nums].map(|nums| nums
                                                                                .split_whitespace()
                                                                                .map(|num| num.parse::<u32>().expect("NaN"))
                                                                            .collect());
    (winning_nums,my_nums)
}

struct ScratchCard {
    position: usize,
    matches: usize,
    visited: bool
}

impl ScratchCard {
    fn new(position: usize, matches: usize, visited: bool) -> Self {
        Self {
            position,
            matches,
            visited 
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("Could not read input.txt in current directory");
    let mut matching_list: Vec<usize> = vec![];
    let mut stack = vec![];
    for line in input.lines() {
        let (winning_nums, my_nums) = parse_line(line);
        matching_list.push(my_nums.clone().iter().filter(|num| winning_nums.contains(num)).count());
    }
    for (i, matches) in matching_list.clone().into_iter().enumerate() {
        stack.push(ScratchCard::new(i, matches, true));
        for j in i+1..i+matches+1 {
            stack.push(ScratchCard::new(j, matching_list[j], false));
        }
    }        
    let mut new_inserted = true;
    while new_inserted {
        new_inserted = false;
        let mut tempstack = vec![];
        for scratchcard in &mut stack {
            if !scratchcard.visited {
                new_inserted = true;
                scratchcard.visited = true;
                for i in scratchcard.position+1..scratchcard.position+scratchcard.matches+1 {
                    tempstack.push(ScratchCard::new(i, matching_list[i], false));
                }
            }
        }
        stack.append(&mut tempstack);
    }
    println!("{:?}", stack.len());
}
