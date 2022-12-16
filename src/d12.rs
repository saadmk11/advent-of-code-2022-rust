use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug)]
struct Square {
    pub x: isize,
    pub y: isize,
}

impl Square {
    pub fn left(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }
    pub fn right(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }
    pub fn up(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }
    pub fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct SquareCost {
    cost: usize,
    square_id: usize,
}

impl Ord for SquareCost {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.square_id.cmp(&other.square_id))
    }
}

impl PartialOrd for SquareCost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part1(input: String) -> i32 {
    let mut start_square: Option<Square> = None;
    let mut end_square: Option<Square> = None;

    let data: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start_square = Some(Square {
                            x: x as isize,
                            y: y as isize,
                        });
                        'a'
                    } else if c == 'E' {
                        end_square = Some(Square {
                            x: x as isize,
                            y: y as isize,
                        });
                        'z'
                    } else {
                        c
                    }
                })
                .collect()
        })
        .collect();

    let grid_width: usize = data[0].len();
    let grid_height: usize = data.len();

    let build_square_id = |p: &Square| p.x as usize + grid_width * p.y as usize;

    let start_square_id: usize = build_square_id(&start_square.unwrap());
    let end_square_id: usize = build_square_id(&end_square.unwrap());

    let mut heap: BinaryHeap<SquareCost> = BinaryHeap::new();
    let mut cost_tracker: Vec<usize> = vec![99999; (grid_height * grid_width) as usize];

    heap.push(SquareCost {
        cost: 0,
        square_id: start_square_id,
    });
    cost_tracker[start_square_id] = 0;

    while let Some(SquareCost { cost, square_id }) = heap.pop() {
        if square_id == end_square_id {
            return cost as i32;
        }

        let current_square: Square = Square {
            x: (square_id % grid_width) as isize,
            y: (square_id / grid_width) as isize,
        };

        for square in [
            current_square.left(),
            current_square.right(),
            current_square.up(),
            current_square.down(),
        ] {
            if (square.x as usize) < grid_width
                && (square.y as usize) < grid_height
                && (data[square.y as usize][square.x as usize] as isize
                    - data[current_square.y as usize][current_square.x as usize] as isize)
                    <= 1
            {
                let new_square_id: usize = build_square_id(&square);
                let new_cost: usize = cost + 1;

                if new_cost < cost_tracker[new_square_id] {
                    let pc: SquareCost = SquareCost {
                        cost: new_cost,
                        square_id: new_square_id,
                    };
                    heap.push(pc);
                    cost_tracker[pc.square_id] = pc.cost;
                }
            }
        }
    }
    0
}

pub fn part2(input: String) -> i32 {
    let mut end_square: Option<Square> = None;

    let data: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        'a'
                    } else if c == 'E' {
                        end_square = Some(Square {
                            x: x as isize,
                            y: y as isize,
                        });
                        'z'
                    } else {
                        c
                    }
                })
                .collect()
        })
        .collect();

    let grid_width: usize = data[0].len();
    let grid_height: usize = data.len();

    let build_square_id = |p: &Square| p.x as usize + grid_width * p.y as usize;

    let mut heap: BinaryHeap<SquareCost> = BinaryHeap::new();
    let mut cost_tracker: Vec<usize> = vec![99999; (grid_height * grid_width) as usize];

    let end_square_id: usize = build_square_id(&end_square.unwrap());
    let mut squares = vec![];

    for y in 0..grid_height {
        for x in 0..grid_width {
            if data[y as usize][x as usize] == 'a' {
                squares.push(Square {
                    x: x as isize,
                    y: y as isize,
                })
            }
        }
    }

    for start_square in squares {
        let start_square_id = build_square_id(&start_square);
        heap.push(SquareCost {
            cost: 0,
            square_id: start_square_id,
        });
        cost_tracker[start_square_id] = 0;
    }

    while let Some(SquareCost { cost, square_id }) = heap.pop() {
        if square_id == end_square_id {
            return cost as i32;
        }

        let current_square: Square = Square {
            x: (square_id % grid_width) as isize,
            y: (square_id / grid_width) as isize,
        };

        for square in [
            current_square.left(),
            current_square.right(),
            current_square.up(),
            current_square.down(),
        ] {
            if (square.x as usize) < grid_width
                && (square.y as usize) < grid_height
                && (data[square.y as usize][square.x as usize] as isize
                    - data[current_square.y as usize][current_square.x as usize] as isize)
                    <= 1
            {
                let new_square_id: usize = build_square_id(&square);
                let new_cost: usize = cost + 1;

                if new_cost < cost_tracker[new_square_id] {
                    let pc: SquareCost = SquareCost {
                        cost: new_cost,
                        square_id: new_square_id,
                    };
                    heap.push(pc);
                    cost_tracker[pc.square_id] = pc.cost;
                }
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi\n".to_string();
        assert_eq!(part1(input), 31);
    }

    #[test]
    fn test_part2() {
        let input = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi\n".to_string();
        assert_eq!(part2(input), 29);
    }
}
