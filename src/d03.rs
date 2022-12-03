pub fn part1(input: String) -> i32 {
    input
        .lines()
        .map(|x| {
            let mut total = 0;
            let (first, second) = x.split_at(x.len() / 2);
            for s in first.chars() {
                if second.contains(s) {
                    if s.is_uppercase() {
                        total += s as i32 - 64 + 26;
                        break;
                    } else {
                        total += s as i32 - 96;
                        break;
                    }
                }
            }
            total
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
