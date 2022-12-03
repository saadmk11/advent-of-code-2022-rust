use std::fs;

mod d01;
mod d02;
mod d03;

fn read_file(day: u8) -> String {
    fs::read_to_string(format!("inputs/d{}.txt", day)).expect("Could not read file")
}

fn main() {
    println!("Advent of Code 2022!");
    // Day 01
    assert_eq!(74198, d01::part1(read_file(1)));
    assert_eq!(209914, d01::part2(read_file(1)));
    // Day 02
    assert_eq!(11873, d02::part1(read_file(2)));
    assert_eq!(12014, d02::part2(read_file(2)));
    // Day 03
    assert_eq!(7581, d03::part1(read_file(3)));
    assert_eq!(2525, d03::part2(read_file(3)));
}
