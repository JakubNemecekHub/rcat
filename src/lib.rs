use std::{
    error::Error,
    fs,
    io::{self, Read},
};

pub enum Config {
    Stdin,
    File(String),
}

impl Config {

    pub fn build(mut args: impl Iterator<Item=String>) -> Result<Config, &'static str> {
        args.next();
        match args.next() {
            Some(value) => {
                if value == "-" {
                    return Ok(Config::Stdin);
                } else {
                    return Ok(Config::File(value));
                }
            },
            None => return Err("Couldn't parse arguments."),
        };
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config {
        Config::Stdin => read_stdin(),
        Config::File(file_path) => read_file(&file_path),
    }
}

pub fn read_file(file_path: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    println!("{}", contents);
    Ok(())
}

pub fn read_stdin() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    println!("{}", buffer);
    Ok(())
}
