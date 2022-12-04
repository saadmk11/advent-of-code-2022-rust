use std::collections::HashSet;

pub fn part1(input: String) -> i32 {
    input
        .lines()
        .filter(|x| {
            let (first, second) = x.split_once(",").unwrap();
            let first = first.split_once("-").unwrap();
            let second = second.split_once("-").unwrap();
            let first_set =
                HashSet::<i32>::from_iter(first.0.parse().unwrap()..=first.1.parse().unwrap());
            let second_set =
                HashSet::<i32>::from_iter(second.0.parse().unwrap()..=second.1.parse().unwrap());
            first_set.is_subset(&second_set) || second_set.is_subset(&first_set)
        })
        .count() as i32
}

pub fn part2(input: String) -> i32 {
    input
        .lines()
        .filter(|x| {
            let (first, second) = x.split_once(",").unwrap();
            let first = first.split_once("-").unwrap();
            let second = second.split_once("-").unwrap();
            let first_set =
                HashSet::<i32>::from_iter(first.0.parse().unwrap()..=first.1.parse().unwrap());
            let second_set =
                HashSet::<i32>::from_iter(second.0.parse().unwrap()..=second.1.parse().unwrap());
            !first_set.is_disjoint(&second_set)
        })
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8\n".to_string();
        assert_eq!(part1(input), 2);
    }

    #[test]
    fn test_part2() {
        let input = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8\n".to_string();
        assert_eq!(part2(input), 4);
    }
}
