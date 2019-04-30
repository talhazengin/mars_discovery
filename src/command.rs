use std::str::FromStr;
use std::io::{Error, ErrorKind};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Command{
    TurnLeft,
    TurnRight,
    Move
}

impl FromStr for Command{
    type Err = Error;

    fn from_str(inp: &str) -> Result<Self, Self::Err> {
        match inp.trim() {
            "M" | "m" => {
                Ok(Command::Move)
            }
            "L" | "l" => {
                Ok(Command::TurnLeft)
            }
            "R" | "r" => {
                Ok(Command::TurnRight)
            }
            _ => {
                let err_msg = format!("Unexpected command {}!", inp);
                return Err(Error::new(ErrorKind::InvalidInput, err_msg.as_ref()))
            }
        }
    }
}

pub fn build_rover_commands(inp: &str) -> Result<Vec<Command>, Error> {
    let mut ret = Vec::new();
    for i in 0..inp.len(){
        ret.push(Command::from_str(&inp[i..i+1])?);
    }

    Ok(ret)
}

#[test]
fn test_build_rover_commands(){
    let input = "MLRR";

    let expected = vec![Command::Move, Command::TurnLeft, Command::TurnRight, Command::TurnRight];

    let output = build_rover_commands(input).unwrap();

    assert_eq!(expected, output);
}
