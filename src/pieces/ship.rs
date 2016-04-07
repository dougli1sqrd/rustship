use std::fmt;

use board::Coordinate;
use board::Orientation;
use board::Hit;
use board::HitStatus;

pub enum ShipType {
    Battleship,
    Carrier,
    Submarine,
    Destroyer,
    Patrol
}

#[derive(Copy, Clone)]
pub struct Ship<'a> {
    ship_data: ShipTypeData<'a>,
    position: Coordinate,
    pub orientation: Orientation,
    hits: usize
}

#[derive(Copy, Clone)]
struct ShipTypeData<'a> {
    symbol: char,
    name: & 'a str,
    size: usize
}

impl<'a> ShipTypeData<'a> {
    pub fn new(ship_type: ShipType) -> ShipTypeData<'a> {
        match ship_type {
            ShipType::Battleship => ShipTypeData {
                symbol: 'B',
                name: "Battleship",
                size: 4
            },
            ShipType::Carrier => ShipTypeData {
                symbol: 'C',
                name: "Carrier",
                size: 5
            },
            ShipType::Destroyer => ShipTypeData {
                symbol: 'D',
                name: "Destroyer",
                size: 3
            },
            ShipType::Submarine => ShipTypeData {
                symbol: 'S',
                name: "Submarine",
                size: 3
            },
            ShipType::Patrol => ShipTypeData {
                symbol: 'P',
                name: "Patrol",
                size: 2
            }
        }
    }
}

impl<'a> Ship<'a> {
    pub fn new(ship_type: ShipType) -> Ship<'a> {
        Ship {
            ship_data: ShipTypeData::new(ship_type),
            position: Coordinate::new(0, 0),
            orientation: Orientation::Horizontal,
            hits: 0
        }
    }

    pub fn coordinates(&self) -> Vec<Coordinate> {
        let mut coordinates: Vec<Coordinate> = Vec::new();

        let start = self.position;
        for i in 0..self.ship_data.size {
            coordinates.push(start.displace(i as isize, self.orientation));
        }
        return coordinates;
    }

    pub fn symbol(&self) -> char {
        self.ship_data.symbol
    }

    pub fn rotate(& mut self) -> & Ship {
        self.orientation = self.orientation.rotate();
        return self;
    }

    pub fn change_pos(& mut self, position: Coordinate) -> & Ship {
        self.position = position;
        return self;
    }

    pub fn get_position(&self) -> Coordinate {
        return self.position;
    }
}

impl<'a> Hit for Ship<'a> {
    fn hits(&self, coordinate: Coordinate) -> HitStatus {
        match self.coordinates().contains(&coordinate) {
            true => HitStatus::Hit,
            false => HitStatus::Miss
        }
    }

    #[allow(unused_variables)]
    fn resolve_hit(& mut self, coordinate: Coordinate) {
        self.hits += 1;
    }
}

impl<'a> fmt::Debug for Ship<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let coordinates = self.coordinates();
        let mut coordinates_str = String::new();
        for c in coordinates {
            coordinates_str.push_str(&c.to_string());
        }
        write!(f, "{}", coordinates_str)
    }
}
