use helpers::read_file;

mod helpers;

mod d1;
mod d10;
mod d11;
mod d12;
mod d13;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;
mod d8;
mod d9;

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
    // Day 06
    assert_eq!(1356, d6::part1(read_file(6)));
    assert_eq!(2564, d6::part2(read_file(6)));
    // Day 07
    assert_eq!(1477771, d7::part1(read_file(7)));
    assert_eq!(3579501, d7::part2(read_file(7)));
    // Day 08
    assert_eq!(1798, d8::part1(read_file(8)));
    assert_eq!(259308, d8::part2(read_file(8)));
    // Day 09
    assert_eq!(6522, d9::part1(read_file(9)));
    assert_eq!(2717, d9::part2(read_file(9)));
    // Day 10
    assert_eq!(13220, d10::part1(read_file(10)));
    assert_eq!("###..#..#..##..#..#.#..#.###..####.#..#.\n#..#.#..#.#..#.#.#..#..#.#..#.#....#.#..\n#..#.#..#.#..#.##...####.###..###..##...\n###..#..#.####.#.#..#..#.#..#.#....#.#..\n#.#..#..#.#..#.#.#..#..#.#..#.#....#.#..\n#..#..##..#..#.#..#.#..#.###..####.#..#.", d10::part2(read_file(10)));
    // Day 11
    assert_eq!(117624, d11::part1(read_file(11)));
    assert_eq!(16792940265, d11::part2(read_file(11)));
    // Day 12
    assert_eq!(520, d12::part1(read_file(12)));
    assert_eq!(508, d12::part2(read_file(12)));
    // Day 13
    assert_eq!(6478, d13::part1(read_file(13)));
    assert_eq!(21922, d13::part2(read_file(13)));
}
