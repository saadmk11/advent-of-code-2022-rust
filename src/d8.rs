pub fn part1(input: String) -> usize {
    let matrix: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let matrix_len: usize = matrix.len();
    let mut visible_trees: usize = (matrix_len * 4) - 4;

    for i in 1..matrix_len - 1 {
        for j in 1..matrix_len - 1 {
            let tree = matrix[i][j];
            // up
            if (0..i).all(|k| tree > matrix[k][j]) {
                visible_trees += 1;
                continue;
            }
            // down
            if (i + 1..matrix_len).all(|k| tree > matrix[k][j]) {
                visible_trees += 1;
                continue;
            }
            // left
            if (0..j).all(|k| tree > matrix[i][k]) {
                visible_trees += 1;
                continue;
            }
            // right
            if (j + 1..matrix_len).all(|k| tree > matrix[i][k]) {
                visible_trees += 1;
                continue;
            }
        }
    }
    visible_trees
}

pub fn part2(input: String) -> usize {
    let matrix: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect();

    let matrix_len: usize = matrix.len();
    let mut highest_scenic_score: usize = 0;

    for i in 1..matrix_len - 1 {
        for j in 1..matrix_len - 1 {
            let tree = matrix[i][j];
            let mut total: usize = 1;

            // up
            let mut up_score: usize = 0;
            for k in (0..i).rev() {
                up_score += 1;
                if tree <= matrix[k][j] {
                    break;
                }
            }
            total *= up_score;

            // down
            let mut down_score: usize = 0;
            for k in i + 1..matrix_len {
                down_score += 1;
                if tree <= matrix[k][j] {
                    break;
                }
            }
            total *= down_score;

            // left
            let mut left_score: usize = 0;
            for k in (0..j).rev() {
                left_score += 1;
                if tree <= matrix[i][k] {
                    break;
                }
            }
            total *= left_score;

            // right
            let mut right_score: usize = 0;
            for k in j + 1..matrix_len {
                right_score += 1;
                if tree <= matrix[i][k] {
                    break;
                }
            }
            total *= right_score;

            if total > highest_scenic_score {
                highest_scenic_score = total;
            }
        }
    }
    highest_scenic_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "30373\n25512\n65332\n33549\n35390\n".to_string();
        assert_eq!(part1(input), 21);
    }

    #[test]
    fn test_part2() {
        let input = "30373\n25512\n65332\n33549\n35390\n".to_string();
        assert_eq!(part2(input), 8);
    }
}
