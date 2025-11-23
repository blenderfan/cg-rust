
#[derive(Copy, Clone)]
pub enum CardinalDirection {

    X = 0,
    Y = 1,
    Z = 2
}

pub fn as_index(dir: &CardinalDirection) -> usize {
    *dir as usize
}

pub fn plane_indices(dir: CardinalDirection) -> (usize, usize) {
    match dir {
        CardinalDirection::X => (1, 2),
        CardinalDirection::Y => (0, 2),
        CardinalDirection::Z => (0, 1),
    }
}