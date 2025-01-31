#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Left,  // 180
    Right, // 0
    Up,    // 90
    Down,  // 270
}

#[derive(Clone, Debug)]
pub struct Cell {
    pub can_move_to: bool,
    pub name: String,
    pub visited: bool,
}

impl Cell {
    pub fn is_exit(&self) -> bool {
        self.name == "exit"
    }
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
// Getting this, allows treating any Direction as unrotated PaddedMatrix
pub type AlignedMatrix = PaddedMatrix;

pub fn matrix_is_not_empty(value: &Matrix) -> bool {
    !value.is_empty()
}

/// Returns inner vec count
pub fn matrix_row_dimension(value: &Matrix) -> Dimension {
    value.first().unwrap().len()
}

/// Returns outer vec count
pub fn matrix_col_dimension(value: &Matrix) -> Dimension {
    value.len()
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
        visited: false,
    }
}

pub fn block_padding() -> Cell {
    Cell {
        can_move_to: false,
        name: "block padding".into(),
        visited: false,
    }
}

pub fn path() -> Cell {
    Cell {
        can_move_to: true,
        name: "path".into(),
        visited: false,
    }
}

pub fn exit() -> Cell {
    Cell {
        can_move_to: true,
        name: "exit".into(),
        visited: false,
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

pub fn align_matrix(matrix: PaddedMatrix, direction: Direction) -> AlignedMatrix {
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

pub fn unalign_matrix(matrix: AlignedMatrix, direction: Direction) -> PaddedMatrix {
    match direction {
        Direction::Left => rotate_180(matrix),
        Direction::Right => matrix, // No rotation
        Direction::Up => rotate_270(matrix),
        Direction::Down => rotate_90(matrix),
    }
}

pub fn unalign_position(matrix: &PaddedMatrix, direction: Direction, pos: AlignedPos) -> PaddedPos {
    let (row, col) = pos;
    let rows = matrix.len();
    let cols = if rows > 0 { matrix[0].len() } else { 0 };

    match direction {
        Direction::Left => (rows - 1 - row, cols - 1 - col),
        Direction::Right => (row, col), // No change
        Direction::Up => (rows - 1 - col, row),
        Direction::Down => (col, cols - 1 - row),
    }
}

pub fn create_shadow_matrix_with<T>(value: &Matrix, fill_with: impl Fn(Pos) -> T) -> Vec<Vec<T>> {
    let result = (0..matrix_col_dimension(value))
        .into_iter()
        .map(|rowi| {
            (0..matrix_row_dimension(value))
                .into_iter()
                .map(|coli| fill_with((rowi, coli)))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    result
}

pub fn derive_hide_matrix(value: &Matrix) -> Vec<Vec<bool>> {
    let mut hide_matrix = create_shadow_matrix_with(value, |_| true);

    for (rowi, row) in value.iter().enumerate() {
        for (coli, cell) in row.iter().enumerate() {
            if cell.visited {
                let mut set_visited = |rowi: usize, coli: usize| hide_matrix[rowi][coli] = false;

                set_visited(rowi, coli);
                set_visited(rowi, coli);
                set_visited(rowi - 1, coli);
                set_visited(rowi + 1, coli);
                set_visited(rowi, coli + 1);
                set_visited(rowi, coli - 1);
            }
        }
    }

    hide_matrix
}

pub mod test_mazes {
    #![allow(unused)]

    use super::*;

    pub fn n0() -> UnpaddedMatrix {
        vec![vec![block(), path(), path(), exit()]]
    }

    pub fn n0_start() -> UnpaddedPos {
        (0, 1)
    }

    pub fn symbolize_cell(value: &Cell, hide: bool, current: bool) -> &str {
        if current {
            return "P";
        }

        if hide {
            return "?";
        }

        match value {
            value if value.is_exit() => ".",
            Cell {
                can_move_to: false, ..
            } => "#",
            Cell {
                can_move_to: true, ..
            } => "+",
        }
    }

    pub fn simple_display_matrix(value: &Matrix) {
        let _ = value
            .into_iter()
            .inspect(|v| {
                let _ = v
                    .into_iter()
                    .inspect(|v| print!("{}", symbolize_cell(*v, false, false)))
                    .collect::<Vec<_>>();
                println!();
            })
            .collect::<Vec<_>>();
    }

    pub fn simple_display_discovered_matrix(value: &Matrix, pos: Pos) {
        let hide_matrix = derive_hide_matrix(value);

        for (rowi, row) in value.iter().enumerate() {
            for (coli, cell) in row.iter().enumerate() {
                let hide = hide_matrix[rowi][coli];

                let current = (rowi, coli) == pos;

                print!("{}", symbolize_cell(cell, hide, current));
            }
            println!();
        }
    }
}

pub fn pick_pos(m: &PaddedMatrix, (rowi, coli): Pos) -> &Cell {
    &m[rowi][coli]
}

pub fn pick_pos_mut(m: &mut PaddedMatrix, (rowi, coli): Pos) -> &mut Cell {
    &mut m[rowi][coli]
}

#[derive(Clone, Debug)]
pub enum MovementStateChange {
    CellChanged((Pos, Cell)),
}

pub struct MovementState {
    pub m: PaddedMatrix,
    pub pos: PaddedPos,
    pub ws: Option<WriteSignal<Option<MovementStateChange>>>,
}

pub fn inc_aligned_pos((rowi, coli): AlignedPos, inc: ColI) -> AlignedPos {
    (rowi, coli + inc)
}

impl MovementState {
    pub fn new(m: UnpaddedMatrix, pos: UnpaddedPos) -> Self {
        let m = pad_matrix(m);
        let pos = pad_position(pos);
        let mut s = Self { m, pos, ws: None };
        s.visit_pos();
        s
    }

    /// Checks current position was reachable
    pub fn validate_pos(&self) {
        let pos = pick_pos(&self.m, self.pos);
        assert!(pos.can_move_to);
    }

    /// Set visited mark for current position
    pub fn visit_pos(&mut self) {
        self.validate_pos();
        let pos = pick_pos_mut(&mut self.m, self.pos);
        pos.visited = true;
        self.notify_subscriber(MovementStateChange::CellChanged((
            self.pos,
            pick_pos(&self.m, self.pos).clone(),
        )));
    }

    /// Returns possible step count to direction
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

    /// Returns all possible directions to go to
    pub fn can_move_to_directions(&self) -> Vec<Direction> {
        let mut can_move_to = vec![];

        for d in vec![
            Direction::Left,
            Direction::Right,
            Direction::Up,
            Direction::Down,
        ] {
            let steps_to_direction = self.movement_possibility(d);
            if steps_to_direction > 0 {
                can_move_to.push(d);
            }
        }

        can_move_to
    }

    /// Moves current position to Direction for a number of steps
    pub fn move_to_direction(&mut self, d: Direction, inc: Steps) {
        let pos = align_position(&self.m, d, self.pos);
        let pos = inc_aligned_pos(pos, inc);
        let pos = unalign_position(&self.m, d, pos);
        self.pos = pos;
        self.validate_pos();
        self.visit_pos();
        self.notify_subscriber(MovementStateChange::CellChanged((
            self.pos,
            pick_pos(&self.m, self.pos).clone(),
        )));
    }

    /// Moves current position to Direction for one step
    pub fn move_to_direction_once(&mut self, d: Direction) {
        self.move_to_direction(d, 1);
    }

    pub fn subscribe(&mut self, ws: WriteSignal<Option<MovementStateChange>>) {
        self.ws.replace(ws);
    }

    fn notify_subscriber(&self, msg: MovementStateChange) {
        if let Some(ws) = &self.ws {
            ws.set(Some(msg));
        }
    }
}

pub mod cmd {
    use super::*;

    pub fn input_direction() -> Direction {
        loop {
            let mut input = String::new();

            println!("Enter something:");

            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let input = input.trim();

            let d = match input {
                "l" => Some(Direction::Left),
                "r" => Some(Direction::Right),
                "u" => Some(Direction::Up),
                "d" => Some(Direction::Down),
                _ => None,
            };

            if let Some(d) = d {
                println!("You picked: {d:?}");
                return d;
            } else {
                println!("Invalid direction: {}", input);
            }
        }
    }

    pub fn input_available_direction(can_move_to: Vec<Direction>) -> Direction {
        assert!(!can_move_to.is_empty());

        loop {
            let d = input_direction();

            for _d in &can_move_to {
                if d == *_d {
                    return d;
                } else {
                    println!("{d:?} not available")
                }
            }
        }
    }
}

pub mod web {
    use super::*;

    pub fn can_move_to_cells(s: &MovementState) -> Vec<(Pos, Direction)> {
        let result = s
            .can_move_to_directions()
            .into_iter()
            .map(|d| {
                let (rowi_delta, coli_delta) = match d {
                    Direction::Up => (-1, 0),
                    Direction::Down => (1, 0),
                    Direction::Left => (0, -1),
                    Direction::Right => (0, 1),
                };

                let (rowi, coli) = s.pos;
                let (rowi, coli) = (rowi as isize, coli as isize);
                let (rowi, coli) = (rowi + rowi_delta, coli + coli_delta);
                ((rowi as RowI, coli as ColI), d)
            })
            .collect();

        result
    }
}

pub mod components;
use leptos::prelude::*;
use leptos_router::components::Outlet;
#[allow(unused)]
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes, A},
    path, MatchNestedRoutes,
};

#[component(transparent)]
pub fn MazeRoutes() -> impl MatchNestedRoutes + Clone {
    use components::maze::MazeComponent;

    view! {
        <ParentRoute path=path!("/maze") view=Outlet>
            <Route path=path!("") view=MazeComponent />
        </ParentRoute>
    }
    .into_inner()
}
