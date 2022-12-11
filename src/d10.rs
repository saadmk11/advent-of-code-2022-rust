enum Command {
    Add(i32),
    Noop,
}

impl Command {
    fn cycle_count(&self) -> i32 {
        match self {
            Self::Add(_) => 2,
            Self::Noop => 1,
        }
    }
}

pub fn part1(input: String) -> i32 {
    let mut cycle: i32 = 0;
    let mut register_x: i32 = 1;
    let mut signal_streanth: i32 = 0;
    let command_vec: Vec<Command> = input
        .lines()
        .map(|line| {
            if line.starts_with("noop") {
                Command::Noop
            } else {
                let (_, value) = line.split_once(' ').unwrap();
                Command::Add(value.parse::<i32>().unwrap())
            }
        })
        .collect();

    fn update_signal_streanth(cycle: i32, register_x: i32, signal_streanth: &mut i32) {
        if cycle == 20 || ((cycle - 20) as f32 % 40.0 == 0.0) {
            *signal_streanth += cycle * register_x;
        }
    }

    for cmd in command_vec {
        match cmd {
            Command::Add(value) => {
                (0..cmd.cycle_count()).for_each(|_| {
                    cycle += 1;
                    update_signal_streanth(cycle, register_x, &mut signal_streanth);
                });
                register_x += value;
            }
            Command::Noop => {
                cycle += cmd.cycle_count();
                update_signal_streanth(cycle, register_x, &mut signal_streanth);
            }
        }
    }

    signal_streanth
}

pub fn part2(input: String) -> String {
    let mut cycle: i32 = 0;
    let mut register_x: i32 = 1;
    let mut screen: String = String::new();
    let command_vec: Vec<Command> = input
        .lines()
        .map(|line| {
            if line.starts_with("noop") {
                Command::Noop
            } else {
                let (_, value) = line.split_once(' ').unwrap();
                Command::Add(value.parse::<i32>().unwrap())
            }
        })
        .collect();

    fn draw(cycle: i32, register_x: i32, screen: &mut String) {
        if cycle > 0 && cycle as f32 % 40.0 == 0.0 {
            screen.push('\n');
        }

        let horizontal_position = cycle % 40;

        if horizontal_position >= register_x - 1 && horizontal_position <= register_x + 1 {
            screen.push('#');
        } else {
            screen.push('.');
        }
    }

    for cmd in command_vec {
        match cmd {
            Command::Add(value) => {
                (0..cmd.cycle_count()).for_each(|_| {
                    draw(cycle, register_x, &mut screen);
                    cycle += 1;
                });
                register_x += value;
            }
            Command::Noop => {
                draw(cycle, register_x, &mut screen);
                cycle += cmd.cycle_count();
            }
        }
    }
    screen
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input =
            "addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop".to_string();
        assert_eq!(part1(input), 13140);
    }

    #[test]
    fn test_part2() {
        let input =
            "addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop".to_string();
        let expected = "##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....".to_string();
        assert_eq!(part2(input), expected);
    }
}
