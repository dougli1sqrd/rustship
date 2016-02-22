use std::fmt;

pub trait GridType<T> {
    fn get(&self, column: usize, row: usize) -> T;

    fn place(& mut self, column: usize, row: usize, item: T);

    fn len(&self) -> usize;

    fn normalize(&self, distance: usize) -> usize {
        distance % self.len()
    }
}

pub trait Inside<T> {
    fn inside(&self, item: &T) -> bool;
}

pub struct Grid<T> {
    squares: Vec<Vec<T>>
}

impl<T:Copy> GridType<T> for Grid<T> {
    fn get(&self, column: usize, row: usize) -> T {
        self.squares[self.normalize(column)][self.normalize(row)]
    }

    fn place(& mut self, column: usize, row: usize, item: T) {
        let norm_column = self.normalize(column);
        let norm_row = self.normalize(row);
        self.squares[norm_column][norm_row] = item
    }

    fn len(&self) -> usize {
        self.squares.len()
    }
}

impl<T:ToString+Copy> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut boardstring = String::new();
        for row in 0..self.len() {
            for col in 0..self.len() {
                boardstring.push_str(&self.get(col, row).to_string());
                if col != self.len() - 1 {
                    boardstring.push(' ');
                }
            }
            boardstring.push('\n');
        }
        return f.write_str(&boardstring);
    }
}

impl<T:Clone> Grid<T> {
    pub fn new(size: usize, init: T) -> Grid<T> {
        Grid {
            squares: vec![vec![init; size]; size]
        }
    }
}
