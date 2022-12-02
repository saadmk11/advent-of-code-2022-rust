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
        .map(|x| x.lines().map(|x| x.trim().parse::<i32>().unwrap()).sum()).collect();
    v.sort_by(|a, b| b.cmp(a));
    v[0..3].iter().sum()
}
