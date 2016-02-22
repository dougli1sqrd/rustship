use std::fmt;

use grid::Grid;
use grid::GridType;
use grid::Inside;
use pieces::Ship;
use board::Coordinate;

pub struct Board<'a> {
    pub grid: Grid<char>,
    ships: Vec<Ship<'a>>
}

impl<'a> Board<'a> {
    pub fn new(size: usize) -> Board<'a> {
        Board {
            grid: Grid::new(size, '.'),
            ships: Vec::new()
        }
    }

    pub fn place_ship(& mut self, ship: Ship<'a>) -> Result<(), Vec<Coordinate>> {
        let coordinates = ship.coordinates();
        if self.inside(&ship) {
            self.update_grid(&coordinates, ship);
            self.ships.push(ship);
            return Ok(());
        }
        let mut outside: Vec<Coordinate> = Vec::new();
        for c in coordinates {
            if !self.grid.inside(&c) {
                outside.push(c);
            }
        }
        return Err(outside);
    }

    fn update_grid(& mut self, coordinates: &[Coordinate], ship: Ship<'a>) {
        for c in coordinates {
            self.grid.place(c.col, c.row, ship.symbol());
        }
    }
}

impl<'a> fmt::Display for Board<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.grid.fmt(f)
    }
}
