use std::{
    error::Error,
    fs,
    io::{self, BufRead, Read},
};

pub enum LineNumbers {
    None,
    All(i32),
    Nonblank(i32)
}

pub fn run(files: &Vec<&str>, mut numbers: LineNumbers) -> Result<(), Box<dyn Error>> {
    if files.is_empty() {
        read_stdin(&mut numbers)?;
    }
    for &file in files {
        if file != "-" {
            print_lines(file, &mut numbers);
        } else {
            read_stdin(&mut numbers)?;
        }
    }
    Ok(())
}

/// Read buffered lines from a file
fn read_lines(file_path: &str) -> io::Result<io::Lines<io::BufReader<fs::File>>> {
    let file = fs::File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}

/// Print lines without numbers
fn print_line(line: &str) -> () {
    println!("{}", line);
}

/// Print numbered lines
fn print_line_numbered(line: &str, count: &mut i32) -> () {
    println!("{} {}", count, line);
    *count += 1;
}

/// Print lines, number only nonblank lines
fn print_line_numbered_non_blank(line: &str, count: &mut i32) -> () {
    if line.is_empty() {
        println!("{}", line);
    } else {
        println!("{} {}", count, line);
        *count += 1;
    }
}

fn print_lines(file_path: &str, numbers: &mut LineNumbers) -> () {
    let lines = read_lines(file_path);
    match lines {
        Err(_) => eprintln!("cat: {}: no such file or directory", &file_path),
        Ok(lines) => {
            // TODO: switch for loop and match
            for line in lines.map_while(Result::ok) {
                match numbers {
                    LineNumbers::None => print_line(&line),
                    LineNumbers::All(ref mut counter) => print_line_numbered(&line, counter),
                    LineNumbers::Nonblank(ref mut counter) => print_line_numbered_non_blank(&line, counter),
                }
            }
        }
    }
}

fn read_stdin(numbers: &mut LineNumbers) -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    for line in buffer.lines() {
        match numbers {
            LineNumbers::None => print_line(&line),
            LineNumbers::All(ref mut counter) => print_line_numbered(&line, counter),
            LineNumbers::Nonblank(ref mut counter) => print_line_numbered_non_blank(&line, counter),
        }
    }
    Ok(())
}
