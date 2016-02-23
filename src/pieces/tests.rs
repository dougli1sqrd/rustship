use super::Ship;
use super::ShipType;
use board::Board;
use board::Coordinate;
use board::Orientation;
use grid::Inside;

#[test]
#[allow(unused_variables)]
fn test_ship_new() {
    let s = Ship::new(ShipType::Battleship);
}

#[test]
fn test_ship_inside_board() {
    let s = Ship::new(ShipType::Battleship);
    let b = Board::new(6);
    assert_eq!(true, b.inside(&s));
}

#[test]
fn test_ship_outside_board() {
    let s = Ship::new(ShipType::Battleship);
    let b = Board::new(2);
    assert_eq!(false, b.inside(&s));
}

#[test]
fn test_ship_vertical_inside() {
    let mut s = Ship::new(ShipType::Battleship);
    let b = Board::new(5);
    assert_eq!(true, b.inside(&s.rotate()));
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
