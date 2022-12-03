pub fn part1(input: String) -> i32 {
    input
        .lines()
        .map(|x| {
            let (first, second) = x.split_at(x.len() / 2);
            first.chars().find(|b| second.contains(*b)).unwrap()
        })
        .map(|x| {
            if x.is_uppercase() {
                x as i32 - 64 + 26
            } else {
                x as i32 - 96
            }
        })
        .sum()
}

pub fn part2(input: String) -> i32 {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|g| {
            g[0].chars()
                .find(|b| g[1].contains(*b) && g[2].contains(*b))
                .unwrap()
        })
        .map(|x| {
            if x.is_uppercase() {
                x as i32 - 64 + 26
            } else {
                x as i32 - 96
            }
        })
        .sum()
}
