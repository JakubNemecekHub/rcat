use std::{
    error::Error,
    fs,
    io::{self, BufRead},
};

type FileLines = io::Lines<io::BufReader<fs::File>>;
type StaticStdinLines = io::Lines<io::BufReader<io::StdinLock<'static>>>;

pub enum LineNumbers {
    None,
    All(i32),
    Nonblank(i32)
}

pub fn run(files: &Vec<&str>, mut numbers: LineNumbers) -> Result<(), Box<dyn Error>> {
    if files.is_empty() {
        print_lines(read_stdin_lines(), &mut numbers)?;
    }
    for &file in files {
        if file != "-" {
            if let Err(_) = print_lines(read_file_lines(file), &mut numbers) {
                eprintln!("cat: {}: no such file or directory", file);
            };
        } else {
            print_lines(read_stdin_lines(), &mut numbers)?;
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

fn print_lines<I, E>(lines_result: Result<I, E>, numbers: &mut LineNumbers) -> Result<(), E>
where I: Iterator<Item = io::Result<String>>,
      E: From<io::Error> {
    match lines_result {
        Err(err) => Err(err),
        Ok(lines) => {
            match numbers {
                LineNumbers::None => print_lines_unnumbered(lines),
                LineNumbers::All(ref mut counter) => print_lines_numbered(lines, counter),
                LineNumbers::Nonblank(ref mut counter) => print_lines_numbered_non_blank(lines, counter),
            }
            Ok(())
        },
    }
}

/// Print lines without numbers
fn print_lines_unnumbered<I>(lines: I) -> ()
where I: Iterator<Item = io::Result<String>> {
    for line in lines.map_while(Result::ok) {
        println!("{}", line);
    }
}

/// Print numbered lines
fn print_lines_numbered<I>(lines: I, count: &mut i32) -> ()
where I: Iterator<Item = io::Result<String>> {
    for line in lines.map_while(Result::ok) {
        println!("{} {}", count, line);
        *count += 1;
    }
}

/// Print lines, number only nonblank lines
fn print_lines_numbered_non_blank<I>(lines: I, count: &mut i32) -> ()
where I: Iterator<Item = io::Result<String>> {
    for line in lines.map_while(Result::ok) {
        if line.is_empty() {
            println!("{}", line);
        } else {
            println!("{} {}", count, line);
            *count += 1;
        }
    }
}


