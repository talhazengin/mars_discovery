use std::fmt;
use std::str::FromStr;
use std::io::{Error, ErrorKind};
use super::direction::Direction;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Position{
    x: i32,
    y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y
        }
    }

    pub fn move_to_direction(&mut self, direction : Direction) {
        *self = *self + direction;
    }
}

impl FromStr for Position{
    type Err = Error;

    fn from_str(inp: &str) -> Result<Self, Self::Err> {
        let splitted : Vec<&str>= inp.trim().split_whitespace().collect();
        let x = i32::from_str(splitted[0])
                    .map_err(|err|{
                        let err_msg = format!("Couldn't parse position field x! {}", err);
                        return Error::new(ErrorKind::InvalidInput, err_msg.as_ref())
                    })?;
        let y = i32::from_str(splitted[1])
                    .map_err(|err| {
                        let err_msg = format!("Couldn't parse position field y! {}", err);
                        return Error::new(ErrorKind::InvalidInput, err_msg.as_ref())
                    })?;

        Ok(Position::new(x, y))
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

impl std::ops::Add<Direction> for Position {
    type Output = Position;
    fn add(mut self, rhs: Direction) -> Self::Output{ 
        match rhs{
            Direction::North => {
                self.y += 1;
            },
            Direction::South => {
                self.y -= 1;
            },
            Direction::West => {
                self.x -= 1;
            },
            Direction::East => {
                self.x += 1;
            }
        }
        self
    }
}

#[test]
fn test_position_from_str(){
    let input = "12 34";
    let expected = Position{
        x : 12,
        y : 34
    };

    let out = Position::from_str(input).unwrap();

    assert_eq!(out, expected);
}

#[test]
fn test_position_arithmetic() {
    let mut position = Position{
        x: 3,
        y: 5,
    };

    let direction = Direction::West;

    let expected = Position{
        x: 2,
        y: 5
    };

    position.move_to_direction(direction);

    assert_eq!(expected, position);
}
