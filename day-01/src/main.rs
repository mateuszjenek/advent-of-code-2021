mod sonar_sweep;

use clap::{Arg, App};

fn main() {
    let matches = App::new("Sonar Sweep")
        .author("Mateusz Jenek <mateusz.jenek@gmail.com>")
        .arg(Arg::with_name("type")
            .short("t")
            .long("type")
            .value_name("TYPE")
            .help("Type of change to count")
            .possible_value("increase")
            .possible_value("i")
            .possible_value("decrease")
            .possible_value("d")
            .possible_value("not_changed")
            .possible_value("n")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("window")
            .short("w")
            .long("window")
            .value_name("WINDOW")
            .help("Depth sets' lenght in coparation")
            .takes_value(true)
            .default_value("1")
            .validator(is_a_number))
        .arg(Arg::with_name("file_name")
            .help("File that stores depth measurements")
            .required(true)
            .index(1))
        .get_matches();

    let change_type = matches.value_of("type").unwrap();
    let file_name = matches.value_of("file_name").unwrap();
    let window = usize::try_from(matches.value_of("window").unwrap().parse::<u16>().unwrap()).unwrap();

    let depths = sonar_sweep::read_depths(String::from(file_name));
    match depths {
        Ok(depths) => {
            let changes = sonar_sweep::calculate_changes(depths, window);
            let number_of_occurences = sonar_sweep::count_changes(
                changes, 
                sonar_sweep::DepthChange::from(
                    String::from(change_type)
                )
            );
            println!("{}", number_of_occurences)
        }
        Err(err) => {
            println!("Error occured while reading depths: {}", err.msg)
        }
    }
}

fn is_a_number(value: String) -> Result<(), String> {
    let result = value.parse::<u16>();
    return match result {
        Ok(..) => Ok(()),
        Err(..) => Err(String::from("The value is not a number"))
    }
}
