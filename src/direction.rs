use std::str::FromStr;

use crate::vector::{self, Vector};

pub const DIRS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::East,
    Direction::West,
];

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
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

    pub const fn prepare_scan(&self, size: Vector) -> (Vector, Self) {
        let step = self.rotate_left();
        let start = match self {
            Self::North => size,
            Self::South => Vector::new(-1, -1),
            Self::East => Vector::new(-1, size.y),
            Self::West => Vector::new(size.x, -1),
        };
        (start, step)
    }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "^" => Ok(Self::North),
            "v" | "V" => Ok(Self::South),
            ">" => Ok(Self::East),
            "<" => Ok(Self::West),
            _ => Err(()),
        }
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::North,
            'v' | 'V' => Self::South,
            '>' => Self::East,
            '<' => Self::West,
            _ => panic!(),
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::North => '^',
                Self::South => 'v',
                Self::East => '>',
                Self::West => '<',
            }
        )?;
        Ok(())
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
