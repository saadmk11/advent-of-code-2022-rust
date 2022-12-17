use std::cmp::Ordering;

#[derive(Debug, Eq)]
pub enum Packet {
    Digit(u32),
    List(Vec<Packet>),
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(v1), Self::List(v2)) => v1 == v2,
            (Self::List(v), Self::Digit(n)) => *v == vec![Packet::Digit(*n)],
            (Self::Digit(n), Self::List(v)) => *v == vec![Packet::Digit(*n)],
            (Self::Digit(n1), Self::Digit(n2)) => n1 == n2,
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::List(v1), Packet::List(v2)) => v1.cmp(v2),
            (Packet::List(v), Packet::Digit(n)) => v.cmp(&vec![Packet::Digit(*n)]),
            (Packet::Digit(n), Packet::List(v)) => vec![Packet::Digit(*n)].cmp(&v),
            (Packet::Digit(n1), Packet::Digit(n2)) => n1.cmp(n2),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part1(input: String) -> usize {
    fn parse(part: &str) -> Vec<Packet> {
        let mut stack: Vec<Packet> = Vec::new();
        let mut current_digit: String = String::new();

        for c in part.chars() {
            match c {
                '[' => {
                    stack.push(Packet::List(Vec::new()));
                }
                ']' => {
                    if current_digit.len() > 0 {
                        match stack.last_mut() {
                            Some(Packet::List(v)) => {
                                v.push(Packet::Digit(current_digit.parse().unwrap()));
                                current_digit.clear();
                            }
                            _ => (),
                        }
                    }
                    let last: Packet = stack.pop().unwrap();
                    match stack.last_mut() {
                        Some(Packet::List(v)) => {
                            v.push(last);
                        }
                        None => {
                            stack.push(last);
                        }
                        _ => (),
                    }
                }
                '0'..='9' => {
                    current_digit.push(c);
                }
                ',' => {
                    if current_digit.len() > 0 {
                        match stack.last_mut() {
                            Some(Packet::List(v)) => {
                                v.push(Packet::Digit(current_digit.parse().unwrap()));
                                current_digit.clear();
                            }
                            _ => (),
                        }
                    }
                }
                _ => (),
            }
        }

        stack
    }

    let mut sum_indices: usize = 0;
    let pairs: Vec<Vec<Vec<Packet>>> = input
        .split("\n\n")
        .map(|p| p.lines().map(|l| parse(l)).collect())
        .collect();

    pairs.iter().enumerate().for_each(|(i, pair)| {
        let left = &pair[0];
        let right = &pair[1];

        match left.cmp(&right) {
            Ordering::Less => {
                sum_indices += i + 1;
            }
            _ => (),
        }
    });
    sum_indices
}

pub fn part2(input: String) -> usize {
    fn parse(part: &str) -> Vec<Packet> {
        let mut stack: Vec<Packet> = Vec::new();
        let mut current_digit: String = String::new();

        for c in part.chars() {
            match c {
                '[' => {
                    stack.push(Packet::List(Vec::new()));
                }
                ']' => {
                    if current_digit.len() > 0 {
                        match stack.last_mut() {
                            Some(Packet::List(v)) => {
                                v.push(Packet::Digit(current_digit.parse().unwrap()));
                                current_digit.clear();
                            }
                            _ => (),
                        }
                    }
                    let last: Packet = stack.pop().unwrap();
                    match stack.last_mut() {
                        Some(Packet::List(v)) => {
                            v.push(last);
                        }
                        None => {
                            stack.push(last);
                        }
                        _ => (),
                    }
                }
                '0'..='9' => {
                    current_digit.push(c);
                }
                ',' => {
                    if current_digit.len() > 0 {
                        match stack.last_mut() {
                            Some(Packet::List(v)) => {
                                v.push(Packet::Digit(current_digit.parse().unwrap()));
                                current_digit.clear();
                            }
                            _ => (),
                        }
                    }
                }
                _ => (),
            }
        }

        stack
    }

    let mut pairs: Vec<Packet> = input
        .split("\n\n")
        .flat_map(|p| p.lines())
        .flat_map(|part: &str| parse(part))
        .collect();

    pairs.push(Packet::List(vec![Packet::List(vec![Packet::Digit(2)])]));
    pairs.push(Packet::List(vec![Packet::List(vec![Packet::Digit(6)])]));
    pairs.sort();

    let divider_packet_1 = pairs
        .iter()
        .enumerate()
        .find(|(_, packet)| packet == &&Packet::List(vec![Packet::List(vec![Packet::Digit(2)])]))
        .unwrap();
    let divider_packet_2 = pairs
        .iter()
        .enumerate()
        .find(|(_, packet)| packet == &&Packet::List(vec![Packet::List(vec![Packet::Digit(6)])]))
        .unwrap();

    (divider_packet_1.0 + 1) * (divider_packet_2.0 + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input =
            "[1,1,3,1,1]\n[1,1,5,1,1]\n\n[[1],[2,3,4]]\n[[1],4]\n\n[9]\n[[8,7,6]]\n\n[[4,4],4,4]\n[[4,4],4,4,4]\n\n[7,7,7,7]\n[7,7,7]\n\n[]\n[3]\n\n[[[]]]\n[[]]\n\n[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]".to_string();
        assert_eq!(part1(input), 13);
    }

    #[test]
    fn test_part2() {
        let input =
            "[1,1,3,1,1]\n[1,1,5,1,1]\n\n[[1],[2,3,4]]\n[[1],4]\n\n[9]\n[[8,7,6]]\n\n[[4,4],4,4]\n[[4,4],4,4,4]\n\n[7,7,7,7]\n[7,7,7]\n\n[]\n[3]\n\n[[[]]]\n[[]]\n\n[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]".to_string();
        assert_eq!(part2(input), 140);
    }
}
