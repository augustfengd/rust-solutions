use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number")
                .long("number")
                .short("n")
                .help("number lines")
                .conflicts_with("number-nonblank")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("number-nonblank")
                .long("number-nonblank")
                .short("b")
                .help("Number nonblank lines")
                .conflicts_with("number")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("file").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number-nonblank"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

// implementation is different than in the book.
fn print(config: &Config, b: Box<dyn BufRead>) {
    let mut n = 0;

    for line in b.lines() {
        let text = line.unwrap();

        let head = if config.number_lines {
            n = n + 1;
            format!("{:>6}\t", n)
        } else if config.number_nonblank_lines {
            if !text.is_empty() {
                n = n + 1;
                format!("{:>6}\t", n)
            } else {
                format!("")
            }
        } else {
            format!("")
        };

        println!("{}{}", head, text);
    }
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in &config.files {
        match open(&filename) {
            Err(err) => eprint!("Failed to open {}: {}", filename, err),
            Ok(f) => print(&config, f),
        }
    }
    Ok(())
}
