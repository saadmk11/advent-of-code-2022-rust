pub fn part1(input: String) -> String {
    let (stack, moves) = input.split_once("\n\n").unwrap();
    let mut stack_iter = stack.lines().rev();

    let mut stack_vec: Vec<Vec<char>> = vec![
        vec![];
        stack_iter
            .next()
            .unwrap()
            .trim()
            .chars()
            .last()
            .unwrap()
            .to_digit(10)
            .unwrap() as usize
    ];

    for line in stack_iter {
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c != ' ' {
                stack_vec[i].push(c);
            }
        }
    }

    for line in moves.lines() {
        let line_vec: Vec<&str> = line.split(" ").collect();
        let (move_count, move_from, move_to): (usize, usize, usize) = (
            line_vec[1].parse::<usize>().unwrap(),
            line_vec[3].parse::<usize>().unwrap() - 1,
            line_vec[5].parse::<usize>().unwrap() - 1,
        );
        let from_vec = &mut stack_vec[move_from];
        let moved = from_vec.split_off(from_vec.len() - move_count);
        stack_vec[move_to].extend(moved.iter().rev());
    }

    let mut top_items = String::new();
    stack_vec.iter().for_each(|x| {
        if let Some(v) = x.last() {
            top_items.push(*v);
        }
    });
    top_items
}

pub fn part2(input: String) -> String {
    let (stack, moves) = input.split_once("\n\n").unwrap();
    let mut stack_iter = stack.lines().rev();

    let mut stack_vec: Vec<Vec<char>> = vec![
        vec![];
        stack_iter
            .next()
            .unwrap()
            .trim()
            .chars()
            .last()
            .unwrap()
            .to_digit(10)
            .unwrap() as usize
    ];

    for line in stack_iter {
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c != ' ' {
                stack_vec[i].push(c);
            }
        }
    }

    for line in moves.lines() {
        let line_vec: Vec<&str> = line.split(" ").collect();
        let (move_count, move_from, move_to): (usize, usize, usize) = (
            line_vec[1].parse::<usize>().unwrap(),
            line_vec[3].parse::<usize>().unwrap() - 1,
            line_vec[5].parse::<usize>().unwrap() - 1,
        );
        let from_vec = &mut stack_vec[move_from];
        let moved = from_vec.split_off(from_vec.len() - move_count);
        stack_vec[move_to].extend(moved.iter());
    }

    let mut top_items = String::new();
    stack_vec.iter().for_each(|x| {
        if let Some(v) = x.last() {
            top_items.push(*v);
        }
    });
    top_items
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input =
            "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2\n".to_string();
        assert_eq!(part1(input), "CMZ");
    }

    #[test]
    fn test_part2() {
        let input =
            "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2\n".to_string();
        assert_eq!(part2(input), "MCD");
    }
}
