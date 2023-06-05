pub struct Solution;

type Board = Vec<Vec<i32>>;

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let mut to_change: Vec<(usize, usize)> = Vec::new();
        for (i, _) in board.iter().enumerate() {
            for (j, _) in board.get(i).unwrap().iter().enumerate() {
                let alive_neighbors = Solution::count_alive_neighbors(board, i, j);
                match (board.get(i).unwrap().get(j).unwrap(), alive_neighbors) {
                    (&1, 0 | 1) => to_change.push((i, j)),
                    (&1, 2 | 3) => (),
                    (&1, 4..) => to_change.push((i, j)),
                    (&0, 3) => to_change.push((i, j)),
                    _ => ()
                }
            }
        }

        for (i, j) in to_change {
            board[i][j] = (board[i][j] - 1).abs()
        }
    }

    pub fn count_alive_neighbors(board: &Board, row_i: usize, col_i: usize) -> i32 {
        let mut count_alive = 0;
        for i in [row_i.checked_sub(1), Some(row_i), row_i.checked_add(1)] {
            for j in [col_i.checked_sub(1), Some(col_i), col_i.checked_add(1)] {
                match (i, j) {
                    (Some(row_idx), Some(col_idx)) => {
                        if row_idx == row_i && col_idx == col_i {
                            continue;
                        }
                        match board.get(row_idx) {
                            Some(column) => {
                                count_alive += column.get(col_idx).unwrap_or(&0);
                            },
                            None => continue
                        }
                    },
                    _ => continue
                }
            }

        }
        count_alive
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_generation() {
        let mut input = vec![vec![0,1,0],vec![0,0,1],vec![1,1,1],vec![0,0,0]];
        Solution::game_of_life(&mut input);
        assert!(input == vec![vec![0,0,0],vec![1,0,1],vec![0,1,1],vec![0,1,0]]);
    }

    #[test]
    fn test_next_generation_2() {
        let mut input = vec![vec![1,1],vec![1,0]];
        Solution::game_of_life(&mut input);
        assert!(input == vec![vec![1,1],vec![1,1]]);
    }
}
