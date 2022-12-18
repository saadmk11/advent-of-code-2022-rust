use std::cmp;
use std::collections::HashSet;

pub fn part1(input: String) -> i32 {
    static SAND_SOURCE: (i32, i32) = (500, 0);
    let mut blocked_paths: HashSet<(i32, i32)> = HashSet::new();
    let mut sand_count = 0;

    input.lines().for_each(|line| {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let mut parts_iter = parts.iter();
        let (prev_x, prev_y) = parts_iter.next().unwrap().split_once(',').unwrap();

        let mut prev_x: i32 = prev_x.parse().unwrap();
        let mut prev_y: i32 = prev_y.parse().unwrap();

        parts.iter().for_each(|part| {
            let (x, y) = part.split_once(',').unwrap();
            let x: i32 = x.parse().unwrap();
            let y: i32 = y.parse().unwrap();

            if y != prev_y {
                (cmp::min(y, prev_y)..=cmp::max(y, prev_y)).for_each(|i| {
                    blocked_paths.insert((x, i));
                });
            } else {
                (cmp::min(x, prev_x)..=cmp::max(x, prev_x)).for_each(|i| {
                    blocked_paths.insert((i, y));
                });
            }
            prev_x = x;
            prev_y = y;
        });
    });

    let y_max = blocked_paths.iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap().1;

    loop {
        let (mut x, mut y): (i32, i32) = SAND_SOURCE;

        loop {
            if !blocked_paths.contains(&(x, y + 1)) {
                y += 1;
            } else if !blocked_paths.contains(&(x - 1, y + 1)) {
                x -= 1;
                y += 1;
            } else if !blocked_paths.contains(&(x + 1, y + 1)) {
                x += 1;
                y += 1;
            } else {
                blocked_paths.insert((x, y));
                break;
            }

            if y > y_max {
                return sand_count;
            }
        }
        sand_count += 1;
    }
}

pub fn part2(input: String) -> i32 {
    static SAND_SOURCE: (i32, i32) = (500, 0);
    let mut blocked_paths: HashSet<(i32, i32)> = HashSet::new();
    let mut sand_count = 0;

    input.lines().for_each(|line| {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let mut parts_iter = parts.iter();
        let (prev_x, prev_y) = parts_iter.next().unwrap().split_once(',').unwrap();

        let mut prev_x: i32 = prev_x.parse().unwrap();
        let mut prev_y: i32 = prev_y.parse().unwrap();

        parts.iter().for_each(|part| {
            let (x, y) = part.split_once(',').unwrap();
            let x: i32 = x.parse().unwrap();
            let y: i32 = y.parse().unwrap();

            if y != prev_y {
                (cmp::min(y, prev_y)..=cmp::max(y, prev_y)).for_each(|i| {
                    blocked_paths.insert((x, i));
                });
            } else {
                (cmp::min(x, prev_x)..=cmp::max(x, prev_x)).for_each(|i| {
                    blocked_paths.insert((i, y));
                });
            }
            prev_x = x;
            prev_y = y;
        });
    });

    let y_max = blocked_paths.iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap().1;

    loop {
        let (mut x, mut y): (i32, i32) = SAND_SOURCE;

        loop {
            if blocked_paths.contains(&(x, y)) {
                return sand_count;
            } else if y == y_max + 1 {
                blocked_paths.insert((x, y));
                break;
            } else if !blocked_paths.contains(&(x, y + 1)) {
                y += 1;
            } else if !blocked_paths.contains(&(x + 1, y + 1)) {
                x += 1;
                y += 1;
            } else if !blocked_paths.contains(&(x - 1, y + 1)) {
                x -= 1;
                y += 1;
            } else {
                blocked_paths.insert((x, y));
                break;
            }
        }
        sand_count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9".to_string();
        assert_eq!(part1(input), 24);
    }

    #[test]
    fn test_part2() {
        let input = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9".to_string();
        assert_eq!(part2(input), 93);
    }
}
