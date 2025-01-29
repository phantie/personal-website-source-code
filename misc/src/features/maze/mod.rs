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

pub type Row = Vec<Cell>;
pub type Matrix = Vec<Row>;
pub type UnpaddedMatrix = Matrix;
pub type PaddedMatrix = Matrix;

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
