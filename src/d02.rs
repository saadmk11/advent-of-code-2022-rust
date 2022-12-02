use std::collections::HashMap;

pub fn part1(input: String) -> i32 {
    let choice_points = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);
    input
        .lines()
        .map(|x| {
            let v: Vec<&str> = x.split(" ").collect();
            match (v[0], v[1]) {
                ("A", "X") => 3 + choice_points.get(v[1]).unwrap(),
                ("A", "Y") => 6 + choice_points.get(v[1]).unwrap(),
                ("A", "Z") => 0 + choice_points.get(v[1]).unwrap(),
                ("B", "X") => 0 + choice_points.get(v[1]).unwrap(),
                ("B", "Y") => 3 + choice_points.get(v[1]).unwrap(),
                ("B", "Z") => 6 + choice_points.get(v[1]).unwrap(),
                ("C", "X") => 6 + choice_points.get(v[1]).unwrap(),
                ("C", "Y") => 0 + choice_points.get(v[1]).unwrap(),
                ("C", "Z") => 3 + choice_points.get(v[1]).unwrap(),
                (_, _) => 0,
            }
        })
        .sum()
}

pub fn part2(input: String) -> i32 {
    let choice_points = HashMap::from([("A", 1), ("B", 2), ("C", 3)]);
    let wins = HashMap::from([("A", "B"), ("B", "C"), ("C", "A")]);
    let loses = HashMap::from([("A", "C"), ("B", "A"), ("C", "B")]);
    input
        .lines()
        .map(|x| {
            let v: Vec<&str> = x.split(" ").collect();
            match (v[0], v[1]) {
                ("A", "X") => 0 + choice_points.get(loses.get(v[0]).unwrap()).unwrap(),
                ("A", "Y") => 3 + choice_points.get(v[0]).unwrap(),
                ("A", "Z") => 6 + choice_points.get(wins.get(v[0]).unwrap()).unwrap(),
                ("B", "X") => 0 + choice_points.get(loses.get(v[0]).unwrap()).unwrap(),
                ("B", "Y") => 3 + choice_points.get(v[0]).unwrap(),
                ("B", "Z") => 6 + choice_points.get(wins.get(v[0]).unwrap()).unwrap(),
                ("C", "X") => 0 + choice_points.get(loses.get(v[0]).unwrap()).unwrap(),
                ("C", "Y") => 3 + choice_points.get(v[0]).unwrap(),
                ("C", "Z") => 6 + choice_points.get(wins.get(v[0]).unwrap()).unwrap(),
                (_, _) => 0,
            }
        })
        .sum()
}
