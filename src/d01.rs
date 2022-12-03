pub fn part1(input: String) -> i32 {
    input
        .split("\n\n")
        .map(|x| x.lines().map(|x| x.trim().parse::<i32>().unwrap()).sum())
        .max()
        .unwrap()
}

pub fn part2(input: String) -> i32 {
    let mut v: Vec<i32> = input
        .split("\n\n")
        .map(|x| x.lines().map(|x| x.trim().parse::<i32>().unwrap()).sum())
        .collect();
    v.sort_by(|a, b| b.cmp(a));
    v[0..3].iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input =
            "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n".to_string();
        assert_eq!(part1(input), 24000);
    }

    #[test]
    fn test_part2() {
        let input =
            "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n".to_string();
        assert_eq!(part2(input), 45000);
    }
}
