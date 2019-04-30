use std::io::{stdin, Stdin, Error};
use std::str::FromStr;
use super::position::Position;
use super::direction::Direction;
use super::command::{Command, build_rover_commands};

pub enum InputType {
    UpperRight(Position),
    Rover(Position, Direction, Vec<Command>)
}

pub struct InputIterator {
    current_line_no: u32,
    current_rover_init : Option<(Position, Direction)>,
    stdin : Stdin
}

impl InputIterator {
    pub fn new() -> Self {
        Self {
            current_line_no: 0,
            current_rover_init: None,
            stdin : stdin() 
        }
    }
} 

impl Iterator for InputIterator {
    type Item = Result<InputType, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut line = String::new();

            if let Err(err) = self.stdin.read_line(&mut line){
                return Some(Err(err))
            }

            self.current_line_no += 1;

            if line.trim().is_empty(){
                return None;
            }

            if self.current_line_no == 1{
                match Position::from_str(line.as_ref()) {
                    Ok(pos) => {
                        return Some(Ok(InputType::UpperRight(pos)))
                    },
                    Err(err) => {
                        return Some(Err(err))
                    }
                }
            } else if is_even(self.current_line_no) {
                let splitted : Vec<&str>= line.trim().rsplitn(2, ' ').collect();

                let dir = match Direction::from_str(splitted[0]) {
                    Ok(dir) => {
                        dir
                    },
                    Err(err) => {
                        return Some(Err(err))
                    }
                }; 

                let pos = match Position::from_str(splitted[1]) {
                    Ok(pos) => {
                        pos
                    },
                    Err(err) => {
                        return Some(Err(err))
                    }
                };

                self.current_rover_init = Some((pos, dir));
                continue
            } else {
                match build_rover_commands(line.trim()) {
                    Ok(commands) => {
                        if let Some((pos, dir)) = self.current_rover_init.take() {
                            return Some(Ok(InputType::Rover(pos, dir, commands)))
                        } else {
                            return None
                        }
                    }, 
                    Err(err) => {
                        return Some(Err(err))
                    }
                }
            }
        }
    }
}

fn is_even(i: u32) -> bool { (i & 0x1) != 1 }
