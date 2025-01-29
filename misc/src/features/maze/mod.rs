#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Left,  // 180
    Right, // 0
    Up,    // 90
    Down,  // 270
}

#[derive(Clone)]
pub struct Cell {
    pub can_move_to: bool,
    pub name: String,
}

pub type Steps = usize;
pub type Dimension = usize;
pub type Row = Vec<Cell>;
/// Assume matrix is not empty, minimal matrix is 1x1
pub type Matrix = Vec<Row>;
pub type UnpaddedMatrix = Matrix;
// Valid row index in PaddedMatrix
pub type RowI = usize;
// Valid column index in PaddedMatrix
pub type ColI = usize;
pub type Pos = (RowI, ColI);
// Position in PaddedMatrix
pub type PaddedPos = Pos;
// Position in UnpaddedMatrix
pub type UnpaddedPos = Pos;
// Position in AlignedMatrix
pub type AlignedPos = PaddedPos;
// Indexing by RowI and ColI always points to Cell
pub type PaddedMatrix = Matrix;
// Rotated matrix with respect to a Direction
// Getting this, allows treating any Direction as if it were Direction::Right for PaddedMatrix
pub type AlignedMatrix = PaddedMatrix;

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

pub fn block() -> Cell {
    Cell {
        can_move_to: false,
        name: "block".into(),
    }
}

pub fn block_padding() -> Cell {
    Cell {
        can_move_to: false,
        name: "block padding".into(),
    }
}

pub fn path() -> Cell {
    Cell {
        can_move_to: true,
        name: "path".into(),
    }
}

pub fn pad_matrix(value: UnpaddedMatrix) -> PaddedMatrix {
    // prepends and appends a padding row to matrix and
    // prepends and appends a padding cell to each row
    //
    //         ###
    //  +  ->  #+#
    //         ###
    //
    assert!(valid_dimensions(&value));

    let dim = matrix_row_dimension(&value);

    let pad_with = || block_padding();

    let up_down_row = || (0..dim).into_iter().map(|_| pad_with()).collect::<Row>();

    let m = std::iter::once(up_down_row())
        .chain(value.into_iter())
        .chain(std::iter::once(up_down_row()))
        .map(|row| {
            std::iter::once(pad_with())
                .chain(row.into_iter())
                .chain(std::iter::once(pad_with()))
                .collect::<Row>()
        })
        .collect::<Matrix>();

    m
}

pub fn align_matrix(matrix: PaddedMatrix, direction: Direction) -> AlignedMatrix {
    fn rotate_90(matrix: Matrix) -> Matrix {
        let rows = matrix.len();
        let cols = if rows > 0 { matrix[0].len() } else { 0 };

        (0..cols)
            .map(|col| {
                (0..rows)
                    .rev()
                    .map(|row| matrix[row][col].clone())
                    .collect()
            })
            .collect()
    }

    fn rotate_180(matrix: Matrix) -> Matrix {
        matrix
            .into_iter()
            .rev()
            .map(|row| row.into_iter().rev().collect())
            .collect()
    }

    fn rotate_270(matrix: Matrix) -> Matrix {
        let rows = matrix.len();
        let cols = if rows > 0 { matrix[0].len() } else { 0 };

        (0..cols)
            .rev()
            .map(|col| (0..rows).map(|row| matrix[row][col].clone()).collect())
            .collect()
    }

    match direction {
        Direction::Left => rotate_180(matrix),
        Direction::Right => matrix,
        Direction::Up => rotate_90(matrix),
        Direction::Down => rotate_270(matrix),
    }
}

pub fn pad_position((rowi, coli): UnpaddedPos) -> PaddedPos {
    (rowi + 1, coli + 1)
}

pub fn align_position(matrix: &PaddedMatrix, direction: Direction, pos: PaddedPos) -> AlignedPos {
    let (row, col) = pos;
    let rows = matrix.len();
    let cols = if rows > 0 { matrix[0].len() } else { 0 };

    match direction {
        Direction::Left => (rows - 1 - row, cols - 1 - col),
        Direction::Right => (row, col), // No change
        Direction::Up => (col, rows - 1 - row),
        Direction::Down => (cols - 1 - col, row),
    }
}

pub mod test_mazes {
    #![allow(unused)]

    use super::*;

    pub fn n0() -> UnpaddedMatrix {
        vec![vec![block(), path(), path()]]
    }

    pub fn n0_start() -> UnpaddedPos {
        (0, 1)
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

pub fn pick_pos(m: &PaddedMatrix, (rowi, coli): Pos) -> &Cell {
    &m[rowi][coli]
}

pub struct MovementState {
    pub m: PaddedMatrix,
    pub pos: PaddedPos,
}

pub fn inc_aligned_pos((rowi, coli): AlignedPos, inc: ColI) -> AlignedPos {
    (rowi, coli + inc)
}

impl MovementState {
    pub fn new(m: UnpaddedMatrix, pos: UnpaddedPos) -> Self {
        let m = pad_matrix(m);
        let pos = pad_position(pos);
        Self { m, pos }
    }

    pub fn validate_init(&self) {
        let pos = pick_pos(&self.m, self.pos);
        assert!(pos.can_move_to);
    }

    pub fn movement_possibility(&self, d: Direction) -> Steps {
        let m = self.m.clone();
        let m = align_matrix(m, d);

        let mut result = 0;

        loop {
            let pos = align_position(&self.m, d, self.pos);
            let pos = inc_aligned_pos(pos, result + 1);
            let cell = pick_pos(&m, pos);

            if cell.can_move_to {
                result += 1;
            } else {
                return result;
            }
        }
    }
}
