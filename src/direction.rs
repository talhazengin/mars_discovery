use std::fmt;
use std::str::FromStr;
use std::io::{Error, ErrorKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    West,
    East,
    South
}

impl FromStr for Direction{
    type Err = Error;

    fn from_str(inp: &str) -> Result<Self, Self::Err> {
        match  inp.trim(){
            "N" | "n" => {
                Ok(Direction::North)
            }
            "W" | "w" => {
                Ok(Direction::West)
            }
            "E" | "e" => {
                Ok(Direction::East)
            }
            "S" | "s" => {
                Ok(Direction::South)
            }
            _ => {
                let err_msg = format!("Couldn't parse direction! Direction should be one of [N, W, E, S]. Found {}", inp);
                return Err(Error::new(ErrorKind::InvalidInput, err_msg.as_ref()))
            }
        }
    }
}

impl Direction {
    pub fn turn_left(&mut self) {
        *self = match self {
            Direction::North => {
                Direction::West
            }
            Direction::West => {
                Direction::South
            }
            Direction::South => {
                Direction::East
            }
            Direction::East => {
                Direction::North
            }
        }
    }

    pub fn turn_right(&mut self) {
        *self = match self {
            Direction::North => {
                Direction::East
            }
            Direction::East => {
                Direction::South
            }
            Direction::South => {
                Direction::West
            }
            Direction::West => {
                Direction::North
            }
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::North => {write!(f, "N")}
            Direction::South => {write!(f, "S")}
            Direction::West  => {write!(f, "W")}
            Direction::East  => {write!(f, "E")}
        }
    }
}

#[test]
fn test_direction_from_str() {
    let input = "W";
    let expected = Direction::West;
    let out = Direction::from_str(input).unwrap();
    assert_eq!(expected, out);
}