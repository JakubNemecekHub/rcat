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
pub enum LineNumbers {
    None,
    All(i32),
    Nonblank(i32)
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


pub fn run(files: &Vec<&str>, mut config: Config) -> Result<(), Box<dyn Error>> {
    if files.is_empty() {
        print_lines(read_stdin_lines(), &mut config)?;
    }
    for &file in files {
        if file != "-" {
            if let Err(_) = print_lines(read_file_lines(file), &mut config) {
                eprintln!("cat: {}: no such file or directory", file);
            };
        } else {
            print_lines(read_stdin_lines(), &mut config)?;
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

fn print_lines<I, E>(lines_result: Result<I, E>, config: &mut Config) -> Result<(), E>
where I: Iterator<Item = io::Result<String>>,
      E: From<io::Error> {
    match lines_result {
        Err(err) => Err(err),
        Ok(lines) => {
            match config.line_numbers {
                LineNumbers::None => print_lines_unnumbered(lines, config),
                LineNumbers::All(ref mut counter) => print_lines_numbered(lines, counter, config.squeeze, config.ends),
                LineNumbers::Nonblank(ref mut counter) => print_lines_numbered_non_blank(lines, counter, config.squeeze, config.ends),
            }
            Ok(())
        },
    }
}

/// Print lines without numbers
fn print_lines_unnumbered<I>(lines: I, config: &mut Config) -> ()
where I: Iterator<Item = io::Result<String>> {
    let mut previous_empty = false;
    let suffix = Suffix(config.ends);
    for line in lines.map_while(Result::ok) {
        if config.squeeze & line.is_empty() & previous_empty {
            continue;
        }
        previous_empty = line.is_empty();
        println!("{}{}", line, suffix);
    }
}

/// Print numbered lines
fn print_lines_numbered<I>(lines: I, count: &mut i32, squeeze: bool, ends: bool) -> ()
where I: Iterator<Item = io::Result<String>> {
    let mut previous_empty = false;
    let suffix = Suffix(ends);
    for line in lines.map_while(Result::ok) {
        if squeeze & line.is_empty() & previous_empty {
            continue;
        }
        previous_empty = line.is_empty();
        println!("{} {}{}", count, line, suffix);
        *count += 1;
    }
}

/// Print lines, number only nonblank lines
fn print_lines_numbered_non_blank<I>(lines: I, count: &mut i32, squeeze: bool, ends: bool) -> ()
where I: Iterator<Item = io::Result<String>> {
    let mut previous_empty = false;
    let suffix = Suffix(ends);
    for line in lines.map_while(Result::ok) {
        if squeeze & line.is_empty() & previous_empty {
            continue;
        }
        previous_empty = line.is_empty();
        if line.is_empty() {
            println!("{}{}", line, suffix);
        } else {
            println!("{} {}{}", count, line, suffix);
            *count += 1;
        }
    }
}


