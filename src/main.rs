use std::process;
use clap::{command, Arg, ArgAction};

use rcat::{LineNumbers, Config};

fn main() {

    let matches = command!()
    .author("Jakub Němeček")
    .about("Concatenate FILE(s), or standard input, to standard output")
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
    .arg(
        Arg::new("squeeze")
        .help("suppress repeated empty output lines")
        .short('s')
        .long("squeeze-blank")
        .action(ArgAction::SetTrue)
    )
    .arg(
        Arg::new("ends")
        .help("display $ at end of each line")
        .short('e')
        .long("show-ends")
        .short_alias('E')
        .action(ArgAction::SetTrue)
    )
    .arg(
        Arg::new("unbuffered")
        .help("(ignored)")
        .short('u')
        .action(ArgAction::SetTrue)
    )
    .get_matches();

    let files = matches
        .get_many::<String>("FILE")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>();

    let mut config = Config {
        line_numbers: LineNumbers::None,
        squeeze: matches.get_flag("squeeze"),
        ends: matches.get_flag("ends"),
    };

    if matches.get_flag("numbers") {
        config.line_numbers = LineNumbers::All;
    } else if matches.get_flag("nonblank") {
        config.line_numbers = LineNumbers::Nonblank;
    }

    let exit_code = rcat::run(&files, &config);
    process::exit(exit_code);

}
