use super::orientation::Coordinate;
use super::orientation::Orientation;
use super::board::Board;
use ::grid::Grid;
use ::grid::Inside;
use pieces::Ship;
use pieces::ShipType;
use board::HitStatus;

#[test]
#[allow(unused_variables)]
fn test_coordinate_new() {
    let c = Coordinate::new(2, 3);
}

#[test]
fn test_coordinate_displace_horizontal() {
    let c = Coordinate::new(1, 1);
    let displaced = c.displace(2, Orientation::Horizontal);
    assert_eq!(Coordinate::new(3, 1), displaced);
}

#[test]
fn test_coordinate_displace_vertical() {
    let c = Coordinate::new(1, 5);
    let displaced = c.displace(-2, Orientation::Vertical);
    assert_eq!(Coordinate::new(1, 3), displaced);
}

#[test]
fn test_coordinate_inside_grid() {
    let c = Coordinate::new(2, 2);
    let grid = Grid::new(5, '.');
    assert_eq!(true, grid.inside(&c));
}

#[test]
fn test_coordinate_outside_grid() {
    let c = Coordinate::new(5, 5);
    let grid = Grid::new(5, '.');
    assert_eq!(false, grid.inside(&c));
}

#[test]
fn test_orientation_rotate() {
    let dir = Orientation::Horizontal;
    assert_eq!(Orientation::Vertical, dir.rotate());

    let dir = Orientation::Vertical;
    assert_eq!(Orientation::Horizontal, dir.rotate());
}

#[test]
#[allow(unused_variables)]
fn test_new_board() {
    let b = Board::new(5);
}

#[test]
fn test_place_ship_success() {
    let mut b = Board::new(5);
    let result = b.place_ship(Ship::new(ShipType::Destroyer));
    assert_eq!(true, result.is_ok());
}

#[test]
fn test_place_ship_fail() {
    let mut b = Board::new(2);
    let result = b.place_ship(Ship::new(ShipType::Destroyer));
    assert_eq!(false, result.is_ok());
}

#[test]
#[allow(unused_must_use)]
fn test_place_ship_board_string() {
    let mut b = Board::new(3);
    b.place_ship(Ship::new(ShipType::Destroyer));
    let expected_board =
       "D D D\n\
        . . .\n\
        . . .\n";
    assert_eq!(expected_board, b.to_string());
}

#[test]
fn test_place_ship_fail_result_contents() {
    let mut b = Board::new(3);
    let s = Ship::new(ShipType::Battleship);
    let error = match b.place_ship(s) {
        Err(coordinates) => coordinates,
        _ => Vec::new()
    };
    let error_coordinates = vec![Coordinate::new(3, 0)];
    assert_eq!(error_coordinates, error);
}

#[test]
fn test_board_place() {
    let mut b = Board::new(5);
    let result = b.place(ShipType::Destroyer, Coordinate::new(1, 1), Orientation::Vertical);
    let expected_board =
        ". . . . .\n\
         . D . . .\n\
         . D . . .\n\
         . D . . .\n\
         . . . . .\n";

    assert_eq!(true, result.is_ok());
    assert_eq!(expected_board, b.to_string());
}

#[test]
#[allow(unused_must_use)]
fn test_board_receive_shot_hit() {
    let mut b = Board::new(5);
    b.place(ShipType::Destroyer, Coordinate::new(1, 1), Orientation::Horizontal);
    let result = b.receive_shot(Coordinate::new(2, 1));
    assert_eq!(HitStatus::Hit, result);
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
