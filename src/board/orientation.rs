use std::fmt;

use grid::Grid;
use grid::GridType;
use grid::Inside;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Orientation {
    Horizontal,
    Vertical
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Coordinate {
    pub col: usize,
    pub row: usize,
}

impl Orientation {
    pub fn rotate(self) -> Orientation {
        match self {
            Orientation::Horizontal => Orientation::Vertical,
            Orientation::Vertical => Orientation::Horizontal
        }
    }
}

impl fmt::Debug for Orientation {
    fn fmt(&self, f: & mut fmt::Formatter) -> fmt::Result {
        let orientation_name = match self {
            &Orientation::Horizontal => "Horizontal",
            &Orientation::Vertical => "Vertical"
        };
        f.write_str(orientation_name)
    }
}

impl Coordinate {
    pub fn new(col: usize, row: usize) -> Coordinate {
        Coordinate {
            col: col,
            row: row
        }
    }

    pub fn displace(&self, distance: isize, direction: Orientation) -> Coordinate {
        match direction {
            Orientation::Horizontal => Coordinate::new((self.col as isize + distance) as usize, self.row),
            Orientation::Vertical => Coordinate::new(self.col, (self.row as isize + distance) as usize)
        }
    }
}

impl fmt::Debug for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Coordinate({}, {})", self.col, self.row)
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.col, self.row)
    }
}

impl<T:Copy> Inside<Coordinate> for Grid<T> {
    fn inside(&self, coordinate: &Coordinate) -> bool {
        coordinate.col < self.len() && coordinate.row < self.len()
    }
}
