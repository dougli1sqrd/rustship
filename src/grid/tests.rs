use super::grid::Grid;
use super::grid::GridType;

#[test]
#[allow(unused_variables)]
fn test_grid_new() {
    let grid = Grid::new(5, '.');
}

#[test]
fn test_grid_len() {
    let grid = Grid::new(5, '.');
    assert_eq!(5, grid.len())
}

#[test]
fn test_grid_new_filled_with_dots() {
    let grid = Grid::new(5, '.');
    assert_eq!('.', grid.get(0, 0));
}

#[test]
fn test_grid_is_square_len() {
    let grid = Grid::new(5, '.');
    assert_eq!('.', grid.get(4, 4));
}

#[test]
fn test_grid_place_item() {
    let mut grid = Grid::new(5, '.');
    grid.place(0, 0, 'a');
    assert_eq!('a', grid.get(0, 0))
}

#[test]
fn test_grid_get_out_of_bounds() {
    let mut grid = Grid::new(5, '.');
    grid.place(0, 0, 'a');
    let placed = grid.get(5, 0);
    assert_eq!(grid.get(0, 0), placed);

    let placed = grid.get(5, 5);
    assert_eq!(grid.get(0, 0), placed);
}

#[test]
fn test_grid_place_out_of_bounds() {
    let mut grid = Grid::new(5, '.');
    grid.place(5, 5, 'a');
    assert_eq!('a', grid.get(0, 0));
}

#[test]
fn test_grid_normalize() {
    let grid = Grid::new(5, '.');
    assert_eq!(0, grid.normalize(5));
}

#[test]
fn test_grid_to_string() {
    let grid = Grid::new(2, '.');
    let expected_str =
       ". .\n\
        . .\n";
    assert_eq!(expected_str.to_string(), grid.to_string());
}
