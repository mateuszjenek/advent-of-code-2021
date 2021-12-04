use crate::submarine::Action;
use clap::Arg;
use clap::App;
use crate::submarine::action::factory::Advanced;
use crate::submarine::action::factory::Simple;

mod submarine;

fn main() {
    let matches = App::new("Dive!")
        .author("Mateusz Jenek <mateusz.jenek@gmail.com>")
        .arg(Arg::with_name("type")
            .short("t")
            .long("type")
            .value_name("TYPE")
            .help("Type of change to count")
            .possible_value("simple")
            .possible_value("s")
            .possible_value("advanced")
            .possible_value("a")
            .takes_value(true)
            .required(false)
            .default_value("s"))
        .arg(Arg::with_name("file_name")
            .help("File that stores depth measurements")
            .required(true)
            .index(1))
        .get_matches();

    let file_name = matches.value_of("file_name").unwrap();
    let factory_type = matches.value_of("type").unwrap();
    let actions: Result<Vec<Action>, submarine::Error>;
    match factory_type {
        "advanced" | "a" => {
            actions = submarine::read_actions(String::from(file_name), Advanced{});
        }
        _ => {
            actions = submarine::read_actions(String::from(file_name), Simple{});
        }
    }

    let mut uboat = submarine::Submarine{
        position: submarine::Position{
            forward: 0,
            depth: 0,
            aim: 0
        }
    };

    match actions {
        Ok(actions) => {
            for action in actions {
                uboat.play(action);
            }
            println!("Submarine position(forward={} depth={})", uboat.position.forward, uboat.position.depth);
            println!("Answer: {}", uboat.position.forward*uboat.position.depth);
        }
        Err(err) => println!("Error occured while reading actions: {}", err.msg)
    }
}
