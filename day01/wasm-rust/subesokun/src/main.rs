use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

mod solution;

fn read_puzzle_input(path: &Path) -> std::vec::Vec<u32> {
    let file = File::open(path).unwrap();
    let content = BufReader::new(&file);
    let mut vec = Vec::new();
    for line in content.lines() {
        let number: u32 = line.unwrap().parse().unwrap();
        vec.push(number);
    }
    return vec
}

fn main() {
    let input_path = Path::new("input.txt");
    let puzzle_input = read_puzzle_input(&input_path);

    // let solution_part_1 = solution::run_star1(puzzle_input);
    // println!("Solution to part 1: {}", solution_part_1);

    let solution_part_2 = solution::run_star2(puzzle_input);
    println!("Solution to part 2: {}", solution_part_2);
}