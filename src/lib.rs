use std::{
    error::Error,
    fmt,
    fs,
    io::{self, BufRead},
};

type FileLines = io::Lines<io::BufReader<fs::File>>;
type StaticStdinLines = io::Lines<io::BufReader<io::StdinLock<'static>>>;

pub struct Config {
    pub line_numbers: LineNumbers,
    pub squeeze: bool,
    pub ends: bool,
}
#[derive(PartialEq)]
pub enum LineNumbers {
    None,
    All,
    Nonblank
}

struct Suffix(bool);

impl fmt::Display for Suffix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            true => write!(f, "$"),
            false => Ok(()),
        }
    }
}


pub fn run(files: &Vec<&str>, config: &Config) -> Result<(), Box<dyn Error>> {
    let mut counter: i32 = 0;
    if files.is_empty() {
        counter += print_lines(read_stdin_lines(), config, counter)?;
    }
    for &file in files {
        if file != "-" {
            match print_lines(read_file_lines(file), config, counter) {
                Ok(delta) => counter += delta,
                Err(_) => eprintln!("cat: {}: no such file or directory", file),
            }
        } else {
            counter += print_lines(read_stdin_lines(), config, counter)?;
        }
    }
    Ok(())
}

/// Read buffered lines from a file
fn read_file_lines(file_path: &str) -> io::Result<FileLines> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines())
}

/// Read buffered lines from stdin
fn read_stdin_lines() -> io::Result<StaticStdinLines> {
    let stdin = Box::leak(Box::new(io::stdin())).lock();
    let reader = io::BufReader::new(stdin);
    Ok(reader.lines())
}

fn print_lines<I, E>(lines_result: Result<I, E>, config: &Config, counter: i32) -> Result<i32, E>
where I: Iterator<Item = io::Result<String>>,
      E: From<io::Error> {
    match lines_result {
        Err(err) => Err(err),
        Ok(lines) => {
            let delta = emit(lines, counter, config);
            Ok(delta)
        },
    }
}

/// Print lines
fn emit<I>(lines: I, mut counter: i32, config: &Config) -> i32
where I: Iterator<Item = io::Result<String>> {
    let mut previous_empty = false;
    let suffix = Suffix(config.ends);
    for line in lines.map_while(Result::ok) {
        let current_empty = line.is_empty();
        if config.squeeze & current_empty & previous_empty {
            continue;
        }
        previous_empty = current_empty;
        if !current_empty && config.line_numbers != LineNumbers::None || 
            current_empty && config.line_numbers == LineNumbers::All {
            print!("{} ", counter);
            counter += 1;
        }
        println!("{}{}", line, suffix);
    }
    counter
}


