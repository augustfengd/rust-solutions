use clap::{App, Arg};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust head")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .multiple(true)
                .help("Input file(s)")
                .default_value("-"),
        )
        .arg(
            Arg::with_name("lines")
                .value_name("LINES")
                .short("n")
                .long("lines")
                .help("Number of lines")
                .default_value("10")
            // .conflicts_with("bytes")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("bytes")
                .value_name("BYTES")
                .short("c")
                .long("bytes")
                .help("Number of bytes")
                .conflicts_with("lines")
                .takes_value(true),
        )
        .get_matches();

    let files = matches.values_of_lossy("file").unwrap();

    let lines = matches
        .value_of("lines")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))?
        .unwrap();

    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;

    Ok(Config {
        files,
        lines,
        bytes,
    })
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}

#[test]
fn test_parse_positive_int() {
    // 3 is an OK integer
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    // Any string is an error
    let res = parse_positive_int("foo");

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn print(config: &Config, b: Box<dyn BufRead>) {
    match config {
        Config { bytes: Some(c), .. } => bytes(c, b),
        Config { lines: n, .. } => lines(n, b),
    }
}

fn bytes(c: &usize, mut b: Box<dyn BufRead>) {
    let mut buf = vec![0; *c];

    let n = b.read(&mut buf).unwrap();

    print!("{}", String::from_utf8_lossy(&buf[..n]))
}

fn lines(n: &usize, mut b: Box<dyn BufRead>) {
    let mut buf = String::new();
    for _ in 0..*n {
        _ = b.read_line(&mut buf);
    }
    print!("{}", buf);
}

pub fn run(config: Config) -> MyResult<()> {
    for (i,filename) in config.files.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(f) if config.files.len() > 1 => { println!("==> {} <==", filename); print(&config, f); if i != config.files.len() - 1 {println!()}},
            Ok(f) => print(&config, f),
        }
    }
    Ok(())
}
