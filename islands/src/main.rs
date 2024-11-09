#![allow(unused)]

fn main() {
    println!("Hello, world!");
}

fn get_index(cols: usize, row: usize, col: usize) -> usize {
    (row * cols) + col
}

fn check_bounds(rows: usize, cols: usize, row: usize, col: usize) -> bool {
    row < rows && col < cols
}

fn mark_cell_visited(islands: &mut Vec<u8>, rows: usize, cols: usize, row: usize, col: usize) {
    if !check_bounds(rows, cols, row, col) {
        return;
    }

    if islands[get_index(cols, row, col)] != 1 {
        return;
    }

    // mark it as visited
    islands[get_index(cols, row, col)] = 2;

    // check up
    if row > 0 {
        mark_cell_visited(islands, rows, cols, row - 1, col);
    }

    // check down
    if row + 1 < rows {
        mark_cell_visited(islands, rows, cols, row + 1, col);
    }

    // check left
    if col > 0 {
        mark_cell_visited(islands, rows, cols, row, col - 1);
    }

    // check right
    if col + 1 < cols {
        mark_cell_visited(islands, rows, cols, row, col + 1);
    }

    // check up-left
    if row > 0 && col > 0 {
        mark_cell_visited(islands, rows, cols, row - 1, col - 1);
    }

    // check up-right
    if row > 0 && col + 1 < cols {
        mark_cell_visited(islands, rows, cols, row - 1, col + 1);
    }

    // check down-left
    if row + 1 < rows && col > 0 {
        mark_cell_visited(islands, rows, cols, row + 1, col - 1);
    }

    // check down-right
    if row + 1 < rows && col + 1 < cols {
        mark_cell_visited(islands, rows, cols, row + 1, col + 1);
    }
}

fn count_islands(islands: Vec<u8>, cols: usize, rows: usize) -> usize {
    let mut islands = islands;
    let mut islands_count = 0;
    for row in 0..rows {
        for col in 0..cols {
            let val = islands[get_index(cols, row, col)];
            if val != 1 {
                continue;
            }
            islands_count += 1;
            mark_cell_visited(&mut islands, rows, cols, row, col);
        }
    }
    islands_count
}

#[cfg(test)]
mod tests {
    use super::count_islands;
    #[test]
    fn test_1() {
        let islands = vec![
            1_u8, 1_u8, 0_u8, 1_u8, 0_u8, 1_u8, 1_u8, 0_u8, 0_u8, 1_u8, 0_u8, 1_u8, 0_u8, 0_u8,
            0_u8, 0_u8, 0_u8, 1_u8, 1_u8, 1_u8, 1_u8, 0_u8, 0_u8, 1_u8, 0_u8, 1_u8, 0_u8, 0_u8,
            1_u8, 1_u8, 0_u8, 1_u8, 1_u8, 0_u8, 0_u8, 1_u8, 0_u8, 1_u8, 0_u8, 0_u8, 0_u8, 1_u8,
            1_u8, 1_u8, 0_u8, 0_u8, 0_u8, 0_u8, 1_u8, 1_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
            0_u8, 1_u8, 0_u8, 1_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
            0_u8, 0_u8,
        ];

        let num = count_islands(islands, 6, 12);
        assert_eq!(num, 6);
    }

    #[test]
    fn test_2() {
        let islands = vec![
            0_u8, 0_u8, 0_u8, 0_u8, 1_u8, 0_u8, 0_u8, 1_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
            0_u8, 0_u8, 0_u8, 0_u8, 1_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
            0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        ];

        let num = count_islands(islands, 4, 9);
        assert_eq!(num, 3);
    }

    #[test]
    fn test_3() {
        let islands = vec![
            0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
            0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
            0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
            0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        ];

        let num = count_islands(islands, 6, 8);
        assert_eq!(num, 0);
    }

    #[test]
    fn test_4() {
        let islands = vec![0_u8, 0_u8, 1_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8];

        let num = count_islands(islands, 4, 2);
        assert_eq!(num, 1);
    }

    #[test]
    fn test_5() {
        let islands = vec![
            1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8,
            1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 0_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8,
            1_u8, 1_u8, 1_u8, 1_u8, 0_u8, 1_u8, 0_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8,
            1_u8, 1_u8, 1_u8, 1_u8, 1_u8, 1_u8,
        ];

        let num = count_islands(islands, 6, 8);
        assert_eq!(num, 1);
    }

    #[test]
    fn test_6() {
        let islands = vec![
            0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 1_u8,
            0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 1_u8, 0_u8, 0_u8,
            0_u8, 1_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 1_u8, 0_u8, 0_u8, 0_u8, 1_u8,
            0_u8, 0_u8, 0_u8, 0_u8, 1_u8, 0_u8, 0_u8, 1_u8, 0_u8, 0_u8, 0_u8, 1_u8, 0_u8, 0_u8,
            0_u8, 0_u8, 1_u8, 0_u8, 0_u8, 1_u8, 0_u8, 0_u8, 0_u8, 1_u8, 0_u8, 0_u8, 0_u8, 0_u8,
            1_u8, 0_u8, 0_u8, 1_u8, 0_u8, 0_u8, 1_u8, 1_u8, 1_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
            0_u8, 1_u8, 0_u8, 0_u8, 0_u8, 1_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
            0_u8, 0_u8, 0_u8, 1_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
            0_u8, 1_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 1_u8, 0_u8, 0_u8, 0_u8, 0_u8, 1_u8,
            1_u8, 1_u8, 1_u8, 0_u8, 0_u8, 0_u8, 1_u8, 1_u8, 1_u8, 1_u8, 0_u8, 1_u8, 1_u8, 1_u8,
            1_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 1_u8, 0_u8, 1_u8, 1_u8, 1_u8, 1_u8, 0_u8,
            0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 1_u8, 0_u8, 1_u8, 1_u8, 1_u8, 1_u8, 0_u8, 0_u8, 0_u8,
            0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 1_u8, 1_u8, 1_u8, 1_u8, 0_u8, 1_u8, 0_u8, 0_u8, 0_u8,
            1_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 1_u8, 1_u8, 1_u8, 0_u8, 0_u8, 1_u8, 1_u8,
            0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 1_u8, 0_u8, 0_u8, 0_u8, 1_u8, 1_u8, 0_u8, 0_u8,
            0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 1_u8, 1_u8, 0_u8, 0_u8, 0_u8, 0_u8,
            0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
            0_u8, 0_u8,
        ];

        let num = count_islands(islands, 12, 20);
        assert_eq!(num, 5);
        // assert_eq!(num, 6); will be 6 if we can't access diagonally
    }

    #[test]
    fn test_7() {
        let islands = vec![1_u8];

        let num = count_islands(islands, 1, 1);
        assert_eq!(num, 1);
    }

    #[test]
    fn test_8() {
        let islands = vec![0_u8];

        let num = count_islands(islands, 0, 0);
        assert_eq!(num, 0);
    }
}
