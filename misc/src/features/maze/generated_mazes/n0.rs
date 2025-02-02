use crate::features::maze::*;

pub fn start_pos() -> UnpaddedPos {
    (0, 1)
}

pub fn matrix() -> UnpaddedMatrix {
    vec![vec![block(), start(), path(), path(), exit()]]
}
