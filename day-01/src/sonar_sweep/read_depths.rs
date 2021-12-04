use std::io::{self, BufRead};
use std::fs::File;
use crate::sonar_sweep::Depth;

pub struct Error {
    pub msg: String,
}

pub fn read_depths(file_path: String) -> Result<Vec<Depth>, Error> {
    let file = File::open(file_path);
    match file {
        Ok(file) => {
            let mut depths: Vec<Depth> = Vec::new();
            let lines = io::BufReader::new(file).lines();
            for line in lines {
                match line {
                    Ok(line) => {
                        let depth_value = line.parse::<u16>();
                        match depth_value {
                            Ok(depth_value) => {
                                depths.push(Depth{ value: depth_value})
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
            return Ok(depths);
        },
        Err(..) => {
            return Err(Error{ 
                msg: String::from("Error occured while reading the file")
            })
        }
    }
}