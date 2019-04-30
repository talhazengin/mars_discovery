mod direction;
mod position;
mod rover;
mod input;
mod command;

use input::{InputIterator, InputType};
use rover::Rover;

fn main() {
    let input_iter = InputIterator::new();
    let mut outs = Vec::new();

    for inp_type in input_iter {
        match inp_type.unwrap(){
            InputType::UpperRight(_) => {},
            InputType::Rover(pos, dir, commands) => {
                let mut rover = Rover::new(pos, dir);

                for command in commands{
                    rover.apply_command(command);
                }

                outs.push(format!("{}", rover));
            }
        }
    }

    for out in outs {
        println!("{}", out);
    }
}
