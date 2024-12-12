use crate::vector::{self, Vector};

pub const DIRS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::East,
    Direction::West,
];

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    #[allow(dead_code)]
    pub const fn rotate_right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::South => Self::West,
            Self::East => Self::South,
            Self::West => Self::North,
        }
    }

    #[allow(dead_code)]
    pub const fn rotate_left(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::South => Self::East,
            Self::East => Self::North,
            Self::West => Self::South,
        }
    }

    #[allow(dead_code)]
    pub const fn flip(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
}

pub struct NonCardinalVectorError;

impl TryFrom<Vector> for Direction {
    type Error = NonCardinalVectorError;

    fn try_from(value: Vector) -> Result<Self, Self::Error> {
        match value.normalized() {
            vector::NORTH => Ok(Self::North),
            vector::SOUTH => Ok(Self::South),
            vector::EAST => Ok(Self::East),
            vector::WEST => Ok(Self::West),
            _ => Err(NonCardinalVectorError),
        }
    }
}
