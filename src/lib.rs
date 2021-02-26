use std::process::exit;

pub struct Sudoku {
    pub matrix: [[u32; 9]; 9],
}

impl Sudoku {
    pub fn new(matrix: [[u32; 9]; 9]) -> Self {
        Sudoku { matrix }
    }
    pub fn find(&self, x: usize, y: usize) -> Option<u32> {
        let mut possible = Vec::<u32>::with_capacity(1);
        let rx = (x / 3) * 3;
        let ry = (y / 3) * 3;
        'MAIN_LOOP: for v in 1..=9 {
            for i in self.matrix[x].iter() {
                if *i == v {
                    //println!("i[x] => {} so con... x == {}", v, x);
                    continue 'MAIN_LOOP;
                }
            }
            for i in self.matrix.iter() {
                if i[y] == v {
                    //println!("i[y] => {} so con... y == {}", v, y);
                    continue 'MAIN_LOOP;
                }
            }
            for i in 0..3 {
                for j in 0..3 {
                    if self.matrix[rx + i][ry + j] == v {
                        continue 'MAIN_LOOP;
                    }
                }
            }
            if possible.is_empty() {
                possible.push(v);
            } else {
                return None;
            }
        }
        Some(possible.remove(0))
    }
    pub fn possible(&self, x: usize, y: usize, value: u32) -> bool {
        let rx = (x / 3) * 3;
        let ry = (y / 3) * 3;
        for i in self.matrix[x].iter() {
            if *i == value {
                //println!("i[x] => {} so con... x == {}", v, x);
                return false;
            }
        }
        for i in self.matrix.iter() {
            if i[y] == value {
                //println!("i[y] => {} so con... y == {}", v, y);
                return false;
            }
        }
        for i in 0..3 {
            for j in 0..3 {
                if self.matrix[rx + i][ry + j] == value {
                    return false;
                }
            }
        }
        true
    }

    pub fn easy_solve(&mut self) {
        loop {
            let mut is_end = true;
            for i in 0..9 {
                for j in 0..9 {
                    if self.matrix[i][j] == 0 {
                        if let Some(v) = self.find(i, j) {
                            self.matrix[i][j] = v;
                            is_end = false;
                        }
                    }
                }
            }
            if is_end {
                break;
            }
        }
    }

    pub fn hard_solve(&mut self) {
        for x in 0..9 {
            for y in 0..9 {
                if self.matrix[x][y] == 0 {
                    for v in 1..=9 {
                        if self.possible(x, y, v) {
                            println!("possible {} {} => {}", x, y, v);
                            self.matrix[x][y] = v;
                            self.hard_solve();
                            if !self.is_solved() {
                                self.matrix[x][y] = 0;
                            }
                        }
                    }
                    return;
                }
            }
        }
    }

    pub fn solve(&mut self) -> [[u32; 9]; 9] {
        self.easy_solve();
        if self.is_solved() {
            return self.matrix;
        } else {
            self.hard_solve();
            print!("hard solve");
            return self.matrix;
        }
    }

    pub fn is_solved(&self) -> bool {
        for i in 0..9 {
            for j in 0..9 {
                if self.matrix[i][j] == 0 {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn sudoku_test() {
        let mut sudoku = Sudoku {
            matrix: [
                [4, 2, 0, 1, 7, 6, 0, 0, 0],
                [7, 0, 0, 0, 0, 0, 4, 0, 0],
                [0, 0, 6, 0, 0, 0, 0, 5, 0],
                [0, 0, 3, 7, 0, 0, 0, 0, 5],
                [0, 0, 4, 0, 3, 0, 6, 0, 0],
                [5, 0, 0, 0, 0, 9, 7, 0, 0],
                [0, 4, 0, 0, 0, 0, 8, 0, 0],
                [0, 0, 9, 0, 0, 0, 0, 0, 1],
                [0, 0, 0, 6, 4, 8, 0, 9, 3],
            ],
        };
        sudoku.solve();
        println!("{:?}", sudoku.matrix);
    }
}
