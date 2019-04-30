use std::fmt;
use super::direction::Direction;
use super::position::Position;
use super::command::Command;

pub struct Rover{
    position: Position,
    direction: Direction
}

impl Rover {
    pub fn new(position: Position,
               direction: Direction) -> Self {
        Self {
            position,
            direction
        }
    }

    pub fn apply_command(&mut self, command: Command) {
        match command {
            Command::TurnLeft => {
                self.direction.turn_left();
            },
            Command::TurnRight => {
                self.direction.turn_right();
            },
            Command::Move => {
                self.position.move_to_direction(self.direction);
            }
        }
    }
}

impl fmt::Display for Rover {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.position, self.direction)
    }
}