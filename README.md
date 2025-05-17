# rcat

Rust implementation of the GNU cat shell command.

## Usage

Usage: cat [OPTION]... [FILE]...

### Positional arguments

Names of the files to concatenate. Treat "-" as standard input. If no files are given, read from standard input. E.g. `cat f - g` means: output f's contents, then standard input, then g's contents.

Repeated files are concatenated repeatedly, just as they appear in arguments. Repeated standard input is concatenated only once, as it is consumed by the program.

### Options

- --help: Display help and exit.
- --version: Output version information and exit.
- **-n, --number**: Number all output lines.
- **-b, --number-nonblank**: Number nonempty output lines.
- **-e, --show-ends**: display $ at end of each line, alias -E
- **-s, --squeeze-blank**: suppress repeated empty output lines

#### Not implemented

- **-A, --show-all**: equivalent to -vET
- **-t**: equivalent to -vT
- **-T, --show-tabs**: display TAB characters as ^I
- **-v, --show-nonprinting**: use ^ and M- notation, except for LFD and TAB

## Resources

- [Coding Challenges](https://codingchallenges.fyi/challenges/challenge-cat/)
- [Wikipedia](https://en.wikipedia.org/wiki/Cat_(Unix))
- [Man page](https://linux.die.net/man/1/cat)
- [Online bash shell](https://www.onlinegdb.com/online_bash_shell)
