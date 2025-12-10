// Inspired by https://github.com/burakemir/gauss-jordan-elimination

pub struct GaussJordan {
    pub matrix: Vec<Vec<f64>>,
}

impl GaussJordan {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            matrix: vec![vec![0.0; cols]; rows],
        }
    }

    pub fn from_matrix(matrix: &[Vec<f64>]) -> Self {
        Self {
            matrix: matrix.to_vec(),
        }
    }

    #[allow(dead_code)]
    pub fn insert(&mut self, row: usize, col: usize, value: f64) {
        self.matrix[row][col] = value;
    }

    pub fn pretty_print(&self) {
        for row in &self.matrix {
            println!("{:?}", row);
        }

        for row in &self.matrix {
            for (col, v) in row.iter().enumerate() {
                if col == self.matrix[0].len() - 1 {
                    println!("{}", v);
                    continue;
                }
                let c = match v {
                    0.0 => ' ',
                    1.0 => 'x',
                    _ => '?',
                };
                print!("{c} ");
            }
        }
    }

    fn find_pivot(&self, d: usize) -> Option<usize> {
        (d..self.matrix.len()).find(|&r| self.matrix[r][d] != 0.0)
    }

    pub fn solve_echelon(&mut self) {
        for c in 0..self.matrix.len() {
            if let Some(i) = self.find_pivot(c) {
                for row in i + 1..self.matrix.len() {
                    let factor = self.matrix[row][c] / self.matrix[i][c];
                    for col in c..self.matrix[row].len() {
                        self.matrix[row][col] -= factor * self.matrix[i][col];
                    }
                }

                if c != i {
                    self.matrix.swap(i, c);
                }
            }
        }
    }

    pub fn solve_reduce(&mut self) {
        for c in 0..self.matrix.len() {
            if let Some(i) = self.find_pivot(c) {
                for row in i + 1..self.matrix.len() {
                    let factor = self.matrix[row][c] / self.matrix[i][c];
                    for col in c..self.matrix[row].len() {
                        self.matrix[row][col] -= factor * self.matrix[i][col];
                    }
                }

                if c != i {
                    self.matrix.swap(i, c);
                }

                let factor = 1.0 / self.matrix[c][c];
                for col in c..self.matrix[c].len() {
                    self.matrix[c][col] *= factor;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gauss_elimination_echelon() {
        let matrix = vec![
            vec![1.0, 2.0, 1.0, 10.0],
            vec![2.0, 3.0, 2.0, 12.0],
            vec![3.0, 1.0, 4.0, 11.0],
        ];
        let mut gauss = GaussJordan::from_matrix(&matrix);
        gauss.solve_echelon();
        assert_eq!(
            gauss.matrix,
            vec![
                vec![1.0, 2.0, 1.0, 10.0],
                vec![0.0, -1.0, 0.0, -8.0],
                vec![0.0, 0.0, 1.0, 21.0],
            ]
        )
    }

    #[test]
    fn gauss_elimination_reduce() {
        let matrix = vec![
            vec![1.0, 2.0, 1.0, 10.0],
            vec![2.0, 3.0, 2.0, 12.0],
            vec![3.0, 1.0, 4.0, 11.0],
        ];
        let mut gauss = GaussJordan::from_matrix(&matrix);
        gauss.solve_reduce();
        assert_eq!(
            gauss.matrix,
            vec![
                vec![1.0, 2.0, 1.0, 10.0],
                vec![0.0, 1.0, 0.0, 8.0],
                vec![0.0, 0.0, 1.0, 21.0],
            ]
        )
    }
}
