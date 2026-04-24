use std::{
    fmt,
    fs,
    io,
};

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

pub fn run(files: &Vec<&str>, config: &Config) -> i32 {
    let mut exit_code = 0;
    let mut counter: i32 = 0;
    let targets = if files.is_empty() { vec!["-"]} else { files.to_vec() };
    for &target in &targets {
        if target != "-" {
            match fs::File::open(target) {
                Ok(file) => {
                    let reader = io::BufReader::new(file);
                    counter += emit(reader, counter, &config);
                },
                Err(_) => {
                    exit_code = 1;
                    eprintln!("cat: {}: no such file or directory", target)
                },
            }
        } else {
            counter += emit(io::stdin().lock(), counter, &config);
        }
    }
    return exit_code;
}

fn emit<R: io::BufRead>(reader: R, mut counter: i32, config: &Config) -> i32 {
    let mut previous_empty = false;
    let suffix = Suffix(config.ends);
    for line in reader.lines().map_while(Result::ok) {
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
    return counter;
}
