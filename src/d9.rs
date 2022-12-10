use std::collections::HashSet;

pub fn part1(input: String) -> i32 {
    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);
    let mut tail_seen: HashSet<(i32, i32)> = HashSet::from([tail]);

    input.lines().for_each(|line| {
        let (direction, step) = line.split_once(" ").unwrap();
        let steps: i32 = step.parse().unwrap();

        (0..steps).for_each(|_| {
            match direction {
                "U" => head.0 += 1,
                "D" => head.0 -= 1,
                "R" => head.1 += 1,
                "L" => head.1 -= 1,
                _ => (),
            }

            if !(head == tail
                || (head.0 == tail.0 && (head.1 - tail.1).abs() == 1)
                || (head.1 == tail.1 && (head.0 - tail.0).abs() == 1)
                || (head.0 - tail.0).abs() == 1 && (head.1 - tail.1).abs() == 1)
            {
                if head.0 != tail.0 && head.1 != tail.1 {
                    if (head.0 - tail.0) > 0 {
                        tail.0 += 1;
                    } else if (head.0 - tail.0) < 0 {
                        tail.0 -= 1;
                    }
                    if (head.1 - tail.1) > 0 {
                        tail.1 += 1;
                    } else if (head.1 - tail.1) < 0 {
                        tail.1 -= 1;
                    }
                } else {
                    if head.1 > tail.1 {
                        tail.1 += 1;
                    } else if head.1 < tail.1 {
                        tail.1 -= 1
                    }
                    if head.0 > tail.0 {
                        tail.0 += 1;
                    } else if head.0 < tail.0 {
                        tail.0 -= 1;
                    }
                }
                tail_seen.insert(tail);
            }
        })
    });
    tail_seen.len() as i32
}

pub fn part2(input: String) -> i32 {
    let mut tail_seen: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);
    let mut rope_knots: Vec<(i32, i32)> = vec![(0, 0); 10];

    input.lines().for_each(|line| {
        let (direction, step) = line.split_once(" ").unwrap();
        let steps: i32 = step.parse().unwrap();

        (0..steps).for_each(|_| {
            match direction {
                "U" => rope_knots[0].0 += 1,
                "D" => rope_knots[0].0 -= 1,
                "R" => rope_knots[0].1 += 1,
                "L" => rope_knots[0].1 -= 1,
                _ => (),
            }

            (1..rope_knots.len()).for_each(|i| {
                let head = rope_knots[i - 1];
                let tail = &mut rope_knots[i];

                if !(head == *tail
                    || (head.0 == tail.0 && (head.1 - tail.1).abs() == 1)
                    || (head.1 == tail.1 && (head.0 - tail.0).abs() == 1)
                    || (head.0 - tail.0).abs() == 1 && (head.1 - tail.1).abs() == 1)
                {
                    if head.0 != tail.0 && head.1 != tail.1 {
                        if (head.0 - tail.0) > 0 {
                            tail.0 += 1;
                        } else if (head.0 - tail.0) < 0 {
                            tail.0 -= 1;
                        }
                        if (head.1 - tail.1) > 0 {
                            tail.1 += 1;
                        } else if (head.1 - tail.1) < 0 {
                            tail.1 -= 1;
                        }
                    } else {
                        if head.1 > tail.1 {
                            tail.1 += 1;
                        } else if head.1 < tail.1 {
                            tail.1 -= 1;
                        }
                        if head.0 > tail.0 {
                            tail.0 += 1;
                        } else if head.0 < tail.0 {
                            tail.0 -= 1;
                        }
                    }
                    if i == 9 {
                        tail_seen.insert(*tail);
                    }
                }
            })
        })
    });
    tail_seen.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2\n".to_string();
        assert_eq!(part1(input), 13);
    }

    #[test]
    fn test_part2() {
        let input = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20".to_string();
        assert_eq!(part2(input), 36);
    }
}
