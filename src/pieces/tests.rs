use super::Ship;
use super::ShipType;
use board::Coordinate;
use board::Orientation;
use board::Hit;
use board::HitStatus;

#[test]
#[allow(unused_variables)]
fn test_ship_new() {
    let s = Ship::new(ShipType::Battleship);
}

#[test]
fn test_rotate_ship() {
    let mut s = Ship::new(ShipType::Battleship);
    assert_eq!(Orientation::Vertical, s.rotate().orientation);
}

#[test]
fn test_ship_change_pos() {
    let mut s = Ship::new(ShipType::Battleship);
    s.change_pos(Coordinate::new(1, 0));
    assert_eq!(Coordinate::new(1, 0), s.get_position());
}

#[test]
fn test_ship_symbol() {
    let s = Ship::new(ShipType::Battleship);
    assert_eq!('B', s.symbol());

    let s = Ship::new(ShipType::Carrier);
    assert_eq!('C', s.symbol());

    let s = Ship::new(ShipType::Destroyer);
    assert_eq!('D', s.symbol());

    let s = Ship::new(ShipType::Patrol);
    assert_eq!('P', s.symbol());

    let s = Ship::new(ShipType::Submarine);
    assert_eq!('S', s.symbol());
}

#[test]
fn test_ship_generates_coordinates() {
    let s = Ship::new(ShipType::Patrol);
    let coordinates = s.coordinates();
    assert_eq!(vec![Coordinate::new(0, 0), Coordinate::new(1, 0)], coordinates);
}

#[test]
fn test_ship_hits_when_inside_coordinates() {
    let s = Ship::new(ShipType::Destroyer);
    let hit = s.hits(Coordinate::new(0, 0));
    assert_eq!(HitStatus::Hit, hit);
}

#[test]
fn test_ship_misses_when_outside_coordinates() {
    let s = Ship::new(ShipType::Destroyer);
    let hit = s.hits(Coordinate::new(1, 1));
    assert_eq!(HitStatus::Miss, hit);
}
