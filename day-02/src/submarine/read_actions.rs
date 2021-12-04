use crate::submarine::action::factory::Factory;
use std::io::{self, BufRead};
use std::fs::File;
use crate::submarine::Action;

pub struct Error {
    pub msg: String,
}

pub fn read_actions<T: Factory + Copy>(file_path: String, action_factory: T) -> Result<Vec<Action>, Error> {
    let file = File::open(file_path);
    match file {
        Ok(file) => {
            let mut actions: Vec<Action> = Vec::new();
            let lines = io::BufReader::new(file).lines();
            for line in lines {
                match line {
                    Ok(line) => {
                        let args: Vec<&str> = line.split(' ').collect();
                        if args.len() != 2 {
                            return Err(Error{
                                msg: String::from("Invalid number of arguments in one of the input lines")
                            })
                        }

                        let value = args[1].parse::<i32>();
                        match value {
                            Ok(value) => {
                                match args[0] {
                                    "forward" => actions.push(action_factory.make_forward(value)),
                                    "down" => actions.push(action_factory.make_down(value)),
                                    "up" => actions.push(action_factory.make_up(value)),
                                    _ => return Err(Error{ 
                                        msg: String::from("Undefined action")
                                    })
                                }
                            }
                            Err(..) => {
                                return Err(Error{ 
                                    msg: String::from("Error occured while parsing line to number")
                                })
                            }
                        }
                    }
                    Err(..) => {
                        return Err(Error{ 
                            msg: String::from("Error occured while reading the file line")
                        })
                    }
                }
            }
            return Ok(actions);
        },
        Err(..) => {
            return Err(Error{ 
                msg: String::from("Error occured while reading the file")
            })
        }
    }
}