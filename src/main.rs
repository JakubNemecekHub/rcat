use std::process;

use clap::{command, Arg, ArgAction};

fn main() {

    let matches = command!()
    .author("Jakub Němeček")
    .about("A Rust implementation of GNU cat program.")
    .arg(
        Arg::new("FILE")
            .required(false)
            .action(ArgAction::Append)
    )
    .arg(
        Arg::new("numbers")
            .help("number all output lines")
            .short('n')
            .long("number")
            .action(ArgAction::SetTrue)
    )
    .get_matches();

    let files = matches
        .get_many::<String>("FILE")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>();

    let numbers = matches.get_flag("numbers");

    if let Err(e) = rcat::run(&files, numbers) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

}
