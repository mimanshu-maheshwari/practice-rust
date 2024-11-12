pub struct Solver;

// Definition for a Point.
#[derive(Debug, PartialEq, Eq)]
pub struct Point {
  pub x: i32,
  pub y: i32
}

impl Point {
  #[inline]
  pub fn new(x: i32, y: i32) -> Self {
    Point{x, y}
  }
}

impl Solver {
    pub fn num_islands2(n: i32, m: i32, operators: Vec<Point>) -> Vec<i32> {
        // write your code here
        let n = n as usize; 
        let m = m as usize;
        let len = n * m; 
        let mut set = DisjointSet::new(len);
        let mut grid = vec![vec![0u8; m]; n];

        let mut islands = Vec::with_capacity(operators.len());
        let mut count = 0;
        for op in operators {
            let x = op.x as usize;
            let y = op.y as usize;
            if grid[x][y] == 1 {
                islands.push(count);
                continue;
            }
            grid[x][y] = 1;
            count += 1;
            for (row , col) in Self::neighbors(n, m, x, y) {
                if grid[row][col] == 1 && set.union((row * m) + col, (x * m) + y) {
                    count -= 1;
                }
            }
            islands.push(count);
        }
        islands
    }
    pub fn neighbors(rows:usize, cols: usize, row: usize, col:usize) -> impl Iterator<Item = (usize, usize)> {
        let mut neighbors = Vec::with_capacity(4);
        // up
        if row > 0 {
            neighbors.push((row - 1, col));
        }
        // right
        if col + 1 < cols {
            neighbors.push((row, col + 1));
        }
        // down 
        if row + 1 < rows {
            neighbors.push((row + 1, col));
        }
        // left
        if col > 0 {
            neighbors.push((row, col - 1));
        }
        neighbors.into_iter()
    }
}

/// Union Find algorithm
struct DisjointSet {
    pub parent: Vec<usize>, 
    pub size: Vec<usize>,
}

impl DisjointSet {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }
    pub fn union(&mut self, u:usize, v:usize) -> bool {
        let parent_u = self.find(u);
        let parent_v = self.find(v);
        if parent_u != parent_v {
            if self.size[parent_u] < self.size[parent_v] {
                self.parent[parent_u] = parent_v;
                self.size[parent_v] += self.size[parent_u];
            } else {
                self.parent[parent_v] = parent_u;
                self.size[parent_u] += self.size[parent_v];
            }
            true
        }else {
            false
        }
    }
    pub fn find(&mut self, point:usize) -> usize {
        if self.parent[point] != point {
            self.parent[point] = self.find(self.parent[point]);
        }
        self.parent[point]
    }
}




#[cfg(test)]
mod tests {
    use super::{Solver, Point};

    #[test]
    fn test_1() {

        // Input: n = 4, m = 5, A = [[1,1],[0,1],[3,3],[3,4]]
        // Output: [1,1,2,2]

        // setup
        let (n, m) = (4, 5);
        let a = vec![vec![1,1],vec![0,1],vec![3,3],vec![3,4]];
        let operators = a.into_iter().map(|val| Point::new(val[0], val[1])).collect();
        let expected_output = vec![1,1,2,2];

        // solve
        let actual_output = Solver::num_islands2(n, m, operators);

        // assert
        assert_eq!(expected_output, actual_output);
    }

    #[test]
    fn test_2() {

        // Input: n = 3, m = 3, A = [[0,0],[0,1],[2,2],[2,1]]
        // Output: [1,1,2,2]

        // setup
        let (n, m) = (3, 3);
        let a = vec![vec![0,0],vec![0,1],vec![2,2],vec![2,1]];
        let operators = a.into_iter().map(|val| Point::new(val[0], val[1])).collect();
        let expected_output = vec![1,1,2,2];

        // solve
        let actual_output = Solver::num_islands2(n, m, operators);

        // assert
        assert_eq!(expected_output, actual_output);
    }
}
