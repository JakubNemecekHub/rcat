use std::{
    error::Error,
    fs,
    io::{self, BufRead, Read},
};

pub fn run(files: &Vec<&str>, numbers: bool) -> Result<(), Box<dyn Error>> {
    let mut count = 0;
    if files.is_empty() {
        read_stdin(numbers, &mut count)?;
    }
    for &file in files {
        if file != "-" {
            print_lines(file, numbers, &mut count);
        } else {
            read_stdin(numbers, &mut count)?;
        }
    }
    Ok(())
}


/// Read buffered lines from a file
fn read_lines(file_path: &str) -> io::Result<io::Lines<io::BufReader<fs::File>>> {
    let file = fs::File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}

fn print_line(line: &str) -> () {
    println!("{}", line);
}

fn print_line_numbered(line: &str, count: &mut i32) -> () {
    println!("{} {}", count, line);
    *count += 1;
}

fn print_lines(file_path: &str, numbers: bool, count: &mut i32) -> () {
    let lines = read_lines(file_path);
    match lines {
        Err(_) => eprintln!("cat: {}: no such file or directory", &file_path),
        Ok(lines) => {
            for line in lines.map_while(Result::ok) {
                if numbers {
                    print_line_numbered(&line, count);
                } else {
                    print_line(&line);
                }
            }
        }
    }
}

fn read_stdin(numbers: bool, count: &mut i32) -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    for line in buffer.lines() {
        if numbers {
            println!("{count} {line}");
            *count += 1;
        } else {
            println!("{line}");
        }
    }
    Ok(())
}
