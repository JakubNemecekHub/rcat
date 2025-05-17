use std::process;
use clap::{command, Arg, ArgAction};

use rcat::LineNumbers;

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
            .conflicts_with("nonblank")
    )
    .arg(
        Arg::new("nonblank")
            .help("number nonempty output lines")
            .short('b')
            .long("number-nonblank")
            .action(ArgAction::SetTrue)
            .conflicts_with("numbers")
    )
    .get_matches();

    let files = matches
        .get_many::<String>("FILE")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>();

    let mut numbers = LineNumbers::None;
    if matches.get_flag("numbers") {
        numbers = LineNumbers::All(0);
    } else if matches.get_flag("nonblank") {
        numbers = LineNumbers::Nonblank(0);
    }

    if let Err(e) = rcat::run(&files, numbers) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

}
