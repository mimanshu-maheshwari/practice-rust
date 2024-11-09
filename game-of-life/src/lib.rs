/// The board is made up of an m x n grid of cells,
/// where each cell has an initial state: 
///     live (represented by a 1) 
///     dead (represented by a 0).
/// Each cell interacts with its eight neighbors (horizontal, vertical, diagonal)
/// Rules:
/// 1. Any live cell with fewer than two live neighbors dies as if caused by under-population.
/// 2. Any live cell with two or three live neighbors lives on to the next generation.
/// 3. Any live cell with more than three live neighbors dies, as if by over-population.
/// 4. Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
///
/// Keeping track through variables.
/// 0, 1: Now 0, or dead.
/// 2, 3: Now 1, or live.
pub fn play_game(grid: &mut Vec<Vec<u8>>, iterations: usize) {
    let rows = grid.len();
    let cols = grid[0].len();

    for _ in 0..iterations {
        for row in 0..rows {
            for col in 0..cols {
                let alive_count = neighbors(rows, cols, row, col).filter(|(r, c)| (grid[*r][*c] & 1) == 1).count();
                if (alive_count == 2 && grid[row][col] == 1) || alive_count == 3 {
                    grid[row][col] += 2;
                }
            }
        }

        for row in 0..rows {
            for col in 0..cols {
                grid[row][col] /= 2;
            }
        }
    }

}

/// will return valid rows and columns to the given cell as iterator on `(row: usize, col: usize)`
fn neighbors(rows:usize, cols:usize, row:usize, col:usize) -> impl Iterator<Item = (usize, usize)> {
    let mut neighbors = vec![];
    if row > 0 {
        // up left
        if col > 0 {
            neighbors.push((row - 1, col - 1));
        }

        // up
        neighbors.push((row - 1, col));

        // up right
        if col + 1 < cols {
            neighbors.push((row - 1, col + 1));
        }
    }
    // left 
    if col > 0 {
       neighbors.push((row, col - 1));
    }
    // right
    if col + 1 < cols {
        neighbors.push((row, col + 1));
    }
    if row + 1 < rows {
        // down left
        if col > 0 {
            neighbors.push((row + 1, col - 1));
        }

        // down
        neighbors.push((row + 1, col));

        // down right
        if col + 1 < cols {
            neighbors.push((row + 1, col + 1));
        }
    }
    neighbors.into_iter()
}

#[cfg(test)]
mod tests {
    use super::play_game;
    #[test]
    fn test_1() {
        #[rustfmt::skip]
        let mut initial_state: Vec<Vec<u8>> = vec![
            vec![0, 1, 0],
            vec![0, 0, 1],
            vec![1, 1, 1],
            vec![0, 0, 0],
        ];
        let iterations: usize = 1;

        #[rustfmt::skip]
        let expected_output: Vec<Vec<u8>>  = vec![
            vec![0, 0, 0],
            vec![1, 0, 1],
            vec![0, 1, 1],
            vec![0, 1, 0],
        ];
        play_game(&mut initial_state, iterations);
        assert_eq!(expected_output, initial_state);
    }
    #[test]
    fn test_2() {
        #[rustfmt::skip]
        let mut initial_state: Vec<Vec<u8>> = vec![
            vec![1,1],
            vec![1,0]
        ];
        let iterations: usize = 1;

        #[rustfmt::skip]
        let expected_output: Vec<Vec<u8>>  = vec![
            vec![1,1],
            vec![1,1]
        ];
        play_game(&mut initial_state, iterations);
        assert_eq!(expected_output, initial_state);
    }
}
