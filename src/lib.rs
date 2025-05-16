use std::{error::Error, fs};

pub struct Config {
    pub file_path: String,
}

impl Config {

    pub fn build(mut args: impl Iterator<Item=String>) -> Result<Config, &'static str> {
        args.next();
        let file_path = match args.next() {
            Some(value) => value,
            None => return Err("Didn't get a file path."),
        };
        Ok(Config { file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("{}", contents);
    Ok(())
}
