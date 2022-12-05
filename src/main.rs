use helpers::read_file;

mod helpers;

mod d1;
mod d2;
mod d3;
mod d4;
mod d5;

fn main() {
    println!("Advent of Code 2022!");
    // Day 01
    assert_eq!(74198, d1::part1(read_file(1)));
    assert_eq!(209914, d1::part2(read_file(1)));
    // Day 02
    assert_eq!(11873, d2::part1(read_file(2)));
    assert_eq!(12014, d2::part2(read_file(2)));
    // Day 03
    assert_eq!(7581, d3::part1(read_file(3)));
    assert_eq!(2525, d3::part2(read_file(3)));
    // Day 04
    assert_eq!(530, d4::part1(read_file(4)));
    assert_eq!(903, d4::part2(read_file(4)));
    // Day 05
    assert_eq!("TBVFVDZPN", d5::part1(read_file(5)));
    assert_eq!("VLCWHTDSZ", d5::part2(read_file(5)));
}
