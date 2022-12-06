use std::collections::HashSet;
use std::collections::VecDeque;

pub fn part1(input: String) -> usize {
    static MARKER_COUNT: usize = 4;

    let input_string = input.lines().next().unwrap();
    let mut deque: VecDeque<char> = VecDeque::with_capacity(MARKER_COUNT);

    for (i, c) in input_string.chars().enumerate() {
        if deque.len() == MARKER_COUNT {
            deque.pop_front();
        }

        deque.push_back(c);

        let set: HashSet<&char> = deque.iter().collect();

        if set.len() == MARKER_COUNT {
            return i + 1;
        }
    }
    0
}

pub fn part2(input: String) -> usize {
    static MARKER_COUNT: usize = 14;

    let input_string = input.lines().next().unwrap();
    let mut deque: VecDeque<char> = VecDeque::with_capacity(MARKER_COUNT);

    for (i, c) in input_string.chars().enumerate() {
        if deque.len() == MARKER_COUNT {
            deque.pop_front();
        }

        deque.push_back(c);

        let set: HashSet<&char> = deque.iter().collect();

        if set.len() == MARKER_COUNT {
            return i + 1;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();
        assert_eq!(part1(input), 5);
    }

    #[test]
    fn test_part2() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();
        assert_eq!(part2(input), 23);
    }
}
