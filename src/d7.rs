use std::collections::HashMap;

pub fn part1(input: String) -> i32 {
    static MAX_SIZE: i32 = 100000;

    let mut map: HashMap<String, i32> = HashMap::from([("/".to_string(), 0)]);
    let mut dirs: Vec<&str> = vec!["/"];

    for l in input.lines().skip(1) {
        let cmd: Vec<&str> = l.split(' ').collect();

        match (cmd.get(0), cmd.get(1), cmd.get(2)) {
            (Some(&"$"), Some(&"ls"), None) => (),
            (Some(&"dir"), Some(_), None) => (),
            (Some(&"$"), Some(&"cd"), Some(&"..")) => {
                dirs.pop();
            }
            (Some(&"$"), Some(&"cd"), Some(dir_name)) => {
                dirs.push(dir_name);
            }
            (Some(file_size), Some(_), None) => {
                let file_size: i32 = file_size.parse().unwrap();

                for i in 0..dirs.len() {
                    map.entry(format!("/{}", &dirs[1..i + 1].join("/")))
                        .and_modify(|s| *s += file_size)
                        .or_insert(file_size);
                }
            }
            _ => (),
        }
    }
    map.values().filter(|x| **x <= MAX_SIZE).sum::<i32>()
}

pub fn part2(input: String) -> i32 {
    static TOTAL_SPACE: i32 = 70000000;
    static TOTAL_REQUIRED_SPACE: i32 = 30000000;

    let mut map: HashMap<String, i32> = HashMap::from([("/".to_string(), 0)]);
    let mut dirs: Vec<&str> = vec!["/"];

    for l in input.lines().skip(1) {
        let cmd: Vec<&str> = l.split(' ').collect();

        match (cmd.get(0), cmd.get(1), cmd.get(2)) {
            (Some(&"$"), Some(&"ls"), None) => (),
            (Some(&"dir"), Some(_), None) => (),
            (Some(&"$"), Some(&"cd"), Some(&"..")) => {
                dirs.pop();
            }
            (Some(&"$"), Some(&"cd"), Some(dir_name)) => {
                dirs.push(dir_name);
            }
            (Some(file_size), Some(_), None) => {
                let file_size: i32 = file_size.parse().unwrap();

                for i in 0..dirs.len() {
                    map.entry(format!("/{}", &dirs[1..i + 1].join("/")))
                        .and_modify(|s| *s += file_size)
                        .or_insert(file_size);
                }
            }
            _ => (),
        }
    }

    let space_needed = TOTAL_REQUIRED_SPACE - (TOTAL_SPACE - map["/"]);
    let mut size_vec: Vec<&i32> = Vec::from_iter(map.values());
    size_vec.sort();

    **size_vec
        .iter()
        .find(|item| ***item >= space_needed)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input =
            "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k\n".to_string();
        assert_eq!(part1(input), 95437);
    }

    #[test]
    fn test_part2() {
        let input =
            "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k\n".to_string();
        assert_eq!(part2(input), 24933642);
    }
}
