use std::fmt;

use grid::Grid;
use grid::GridType;
use grid::Inside;
use pieces::Ship;
use pieces::ShipType;
use board::HitStatus;
use board::Coordinate;
use board::Orientation;
use board::Hit;

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

    pub fn place(& mut self, ship_type: ShipType, position: Coordinate, direction: Orientation) -> Result<(), Vec<Coordinate>> {
        let mut ship = Ship::new(ship_type);
        ship.change_pos(position);
        if direction == Orientation::Vertical {
            //rotate the ship object if we want vertical, since a new ship always starts Horizontal
            ship.rotate();
        }
        return self.place_ship(ship);
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

    pub fn receive_shot(& mut self, coordinate: Coordinate) -> HitStatus {
        for ship in &self.ships {
            let ship_spaces = ship.coordinates();
            if ship_spaces.contains(&coordinate) {
                self.grid.place(coordinate.col, coordinate.row, 'x');
                return HitStatus::Hit;
            }
        }
        return HitStatus::Miss;
    }

    fn update_grid(& mut self, coordinates: &[Coordinate], ship: Ship<'a>) {
        for c in coordinates {
            self.grid.place(c.col, c.row, ship.symbol());
        }
    }
}

impl<'a> Inside<Ship<'a>> for Board<'a> {
    fn inside(&self, ship: &Ship) -> bool {
        let coordinates = ship.coordinates();
        for c in coordinates {
            if !self.grid.inside(&c) {
                return false;
            }
        }
        return true;
    }
}

impl<'a> Hit for Board<'a> {
    fn hits(&self, coordinate: Coordinate) -> HitStatus {
        for ship in &self.ships {
            match ship.hits(coordinate) {
                HitStatus::Hit => {
                    return HitStatus::Hit;
                },
                HitStatus::Miss => {}
            };
        }
        return HitStatus::Miss;
    }

    fn resolve_hit(& mut self, coordinate: Coordinate) {

    }
}

impl<'a> fmt::Display for Board<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.grid.fmt(f)
    }
}
