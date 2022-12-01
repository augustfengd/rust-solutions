use std::{error::Error, io::{BufReader, self}};

use clap::{App, Arg};

#[derive(Debug)]
pub struct Config {
    input_file: String,
    output_file: Option<String>,
    count: bool,
}

pub fn get_args() -> Result<Config, Box<dyn Error>> {
    let matches = App::new("uniqr")
        .arg(
            Arg::with_name("IN_FILE")
                .takes_value(true)
                .help("Input file")
                .default_value("-"),
        )
        .arg(
            Arg::with_name("OUT_FILE")
                .help("Output file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("count")
                .short("-c")
                .long("--count")
                .help("Show counts"),
        )
        .get_matches();

    Ok(Config {
        input_file: matches.value_of_lossy("IN_FILE").unwrap().to_string(),
        output_file: matches.value_of("OUT_FILE").map(String::from),
        count: matches.is_present("count"),
    })
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("{:?}", config);
    Ok(())
}
