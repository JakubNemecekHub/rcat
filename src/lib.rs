use std::{
    error::Error,
    fs,
    io::{self, Read},
};

pub enum Config {
    Man,
    Stdin,
    File(Vec<String>),
}

impl Config {

    pub fn build(mut args: impl Iterator<Item=String>) -> Result<Config, &'static str> {
        args.next();
        let all_args: Vec<String> = args.collect();
        if all_args.is_empty() {
            Ok(Config::Man)
        } else if all_args[0] == "-" {
            Ok(Config::Stdin)
        } else {
            Ok(Config::File(all_args))
        }
    }

}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config {
        Config::Man => show_manual(),
        Config::Stdin => read_stdin()?,
        Config::File(file_path) => read_file(&file_path)?,
    }
    Ok(())
}

fn show_manual() -> () {
    println!("Usage: cat [OPTION]... [FILE]...");
    println!("Description: Concatenate FILE(s), or standard input, to standard output.");
}

pub fn read_file(file_paths: &Vec<String>) -> Result<(), Box<dyn Error>> {

    for file_path in file_paths {
        let contents = fs::read_to_string(file_path)?;
        println!("{}", contents);
    }
    Ok(())
}

pub fn read_stdin() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    println!("{}", buffer);
    Ok(())
}
