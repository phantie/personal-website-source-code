pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub struct Cell {
    pub can_move_to: bool,
    pub name: String,
}

pub type Dimension = usize;
pub type Row = Vec<Cell>;
/// Assume matrix is not empty
pub type Matrix = Vec<Row>;
pub type UnpaddedMatrix = Matrix;
pub type PaddedMatrix = Matrix;

pub fn matrix_is_not_empty(value: &Matrix) -> bool {
    !value.is_empty()
}

pub fn matrix_row_dimension(value: &Matrix) -> Dimension {
    value.first().unwrap().len()
}

pub fn row_dimension(value: &Row) -> Dimension {
    value.len()
}

/// returns true if all dimensions are equal
/// returns false otherwise
pub fn valid_dimensions(value: &Matrix) -> bool {
    assert!(matrix_is_not_empty(&value));
    value
        .into_iter()
        .all(|row| row_dimension(row) == matrix_row_dimension(value))
}

pub fn pad_matrix(value: UnpaddedMatrix) -> PaddedMatrix {
    assert!(valid_dimensions(&value));

    // let mut result: Matrix = Default::default();

    unimplemented!()
}

pub mod test_mazes {
    #![allow(unused)]

    use super::*;

    pub fn block() -> Cell {
        Cell {
            can_move_to: false,
            name: "block".into(),
        }
    }

    pub fn path() -> Cell {
        Cell {
            can_move_to: true,
            name: "path".into(),
        }
    }

    pub fn n0() -> UnpaddedMatrix {
        vec![vec![block(), path(), path()]]
    }

    pub fn symbolize_cell(value: &Cell) -> &str {
        match value {
            Cell {
                can_move_to: false, ..
            } => "#",
            Cell {
                can_move_to: true, ..
            } => "+",
        }
    }

    pub fn simple_display_unpadded_matrix(value: &UnpaddedMatrix) {
        let _ = value
            .into_iter()
            .inspect(|v| {
                let _ = v
                    .into_iter()
                    .inspect(|v| print!("{}", symbolize_cell(*v)))
                    .collect::<Vec<_>>();
                println!();
            })
            .collect::<Vec<_>>();
    }
}
