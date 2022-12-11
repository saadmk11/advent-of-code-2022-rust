enum WorryOperation {
    Divide(i64),
    Mod(i64),
}

#[derive(Debug)]
struct Monkey {
    inspection_count: i64,
    items: Vec<i64>,
    div_by: i64,
    true_case: i64,
    false_case: i64,
    worry_level_op_str: String,
}

impl Monkey {
    fn get_new_worry_level(&self, old_worry_level: i64, worry_level_op: &WorryOperation) -> i64 {
        let op_vec: Vec<&str> = self.worry_level_op_str.split(' ').collect();

        let result = match (op_vec[0], op_vec[1], op_vec[2]) {
            (_, "+", "old") => old_worry_level + old_worry_level,
            (_, "-", "old") => old_worry_level - old_worry_level,
            (_, "*", "old") => old_worry_level * old_worry_level,
            (_, "/", "old") => old_worry_level / old_worry_level,
            (_, "+", v) => old_worry_level + v.parse::<i64>().unwrap(),
            (_, "-", v) => old_worry_level - v.parse::<i64>().unwrap(),
            (_, "*", v) => old_worry_level * v.parse::<i64>().unwrap(),
            (_, "/", v) => old_worry_level / v.parse::<i64>().unwrap(),
            _ => panic!("Invalid Operation!"),
        };

        match worry_level_op {
            WorryOperation::Mod(val) => result % *val,
            WorryOperation::Divide(val) => result / *val,
        }
    }

    pub fn inspect(&mut self, worry_level_op: WorryOperation) -> Vec<(i64, i64)> {
        let mut result: Vec<(i64, i64)> = vec![];

        for item in self.items.iter() {
            let new: i64 = self.get_new_worry_level(*item, &worry_level_op);
            if new % self.div_by == 0 {
                result.push((self.true_case, new));
            } else {
                result.push((self.false_case, new));
            }
            self.inspection_count += 1;
        }
        self.items.clear();
        result
    }

    pub fn add_item(&mut self, item: i64) {
        self.items.push(item);
    }
}

pub fn part1(input: String) -> i64 {
    let mut monkey_list: Vec<Monkey> = vec![];

    for note in input.split("\n\n") {
        let mut note_iter = note.lines();
        note_iter.next();

        let items: Vec<i64> = note_iter
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect();
        let operation = note_iter.next().unwrap().split_once("= ").unwrap().1;
        let test: i64 = note_iter
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let true_case: i64 = note_iter
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let false_case: i64 = note_iter
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse()
            .unwrap();

        monkey_list.push(Monkey {
            items: items,
            inspection_count: 0,
            div_by: test,
            true_case: true_case,
            false_case: false_case,
            worry_level_op_str: operation.to_string(),
        });
    }

    let mut result: Vec<(i64, i64)> = vec![];

    for round in 0..21 {
        for (i, monkey) in monkey_list.iter_mut().enumerate() {
            result.retain(|item| {
                if item.0 == i as i64 {
                    monkey.add_item(item.1);
                    false
                } else {
                    true
                }
            });

            if round < 20 {
                result.extend(monkey.inspect(WorryOperation::Divide(3)));
            }
        }
    }
    monkey_list.sort_by(|a, b| b.inspection_count.cmp(&a.inspection_count));
    monkey_list
        .iter()
        .map(|x| x.inspection_count)
        .take(2)
        .product::<i64>()
}

pub fn part2(input: String) -> i64 {
    let mut monkey_list: Vec<Monkey> = vec![];

    for note in input.split("\n\n") {
        let mut note_iter = note.lines();
        note_iter.next();

        let (_, items) = note_iter.next().unwrap().split_once(": ").unwrap();
        let items: Vec<i64> = items.split(", ").map(|x| x.parse().unwrap()).collect();
        let (_, operation) = note_iter.next().unwrap().split_once("= ").unwrap();
        let test: i64 = note_iter
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let true_case: i64 = note_iter
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let false_case: i64 = note_iter
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse()
            .unwrap();

        monkey_list.push(Monkey {
            items: items,
            inspection_count: 0,
            div_by: test,
            true_case: true_case,
            false_case: false_case,
            worry_level_op_str: operation.to_string(),
        });
    }

    let mut result: Vec<(i64, i64)> = vec![];
    let mod_val: i64 = monkey_list.iter().map(|m| m.div_by).product();

    for round in 0..10001 {
        for (i, monkey) in monkey_list.iter_mut().enumerate() {
            result.retain(|item| {
                if item.0 == i as i64 {
                    monkey.add_item(item.1);
                    false
                } else {
                    true
                }
            });

            if round < 10000 {
                result.extend(monkey.inspect(WorryOperation::Mod(mod_val)));
            }
        }
    }
    monkey_list.sort_by(|a, b| b.inspection_count.cmp(&a.inspection_count));
    monkey_list
        .iter()
        .map(|x| x.inspection_count)
        .take(2)
        .product::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input =
            "Monkey 0:\n  Starting items: 79, 98\n  Operation: new = old * 19\n  Test: divisible by 23\n    If true: throw to monkey 2\n    If false: throw to monkey 3\n\nMonkey 1:\n  Starting items: 54, 65, 75, 74\n  Operation: new = old + 6\n  Test: divisible by 19\n    If true: throw to monkey 2\n    If false: throw to monkey 0\n\nMonkey 2:\n  Starting items: 79, 60, 97\n  Operation: new = old * old\n  Test: divisible by 13\n    If true: throw to monkey 1\n    If false: throw to monkey 3\n\nMonkey 3:\n  Starting items: 74\n  Operation: new = old + 3\n  Test: divisible by 17\n    If true: throw to monkey 0\n    If false: throw to monkey 1".to_string();
        assert_eq!(part1(input), 10605);
    }

    #[test]
    fn test_part2() {
        let input =
            "Monkey 0:\n  Starting items: 79, 98\n  Operation: new = old * 19\n  Test: divisible by 23\n    If true: throw to monkey 2\n    If false: throw to monkey 3\n\nMonkey 1:\n  Starting items: 54, 65, 75, 74\n  Operation: new = old + 6\n  Test: divisible by 19\n    If true: throw to monkey 2\n    If false: throw to monkey 0\n\nMonkey 2:\n  Starting items: 79, 60, 97\n  Operation: new = old * old\n  Test: divisible by 13\n    If true: throw to monkey 1\n    If false: throw to monkey 3\n\nMonkey 3:\n  Starting items: 74\n  Operation: new = old + 3\n  Test: divisible by 17\n    If true: throw to monkey 0\n    If false: throw to monkey 1".to_string();
        assert_eq!(part2(input), 2713310158);
    }
}
