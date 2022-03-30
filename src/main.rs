mod cli;

use std::io;
use std::process::exit;
use structopt::StructOpt;
use cli::Cli;

fn main() {
    let cli = Cli::from_args();
    if atty::is(atty::Stream::Stdin) {
        exit(0);
    }
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n == 0 {
                    exit(0);
                }
                match clean_line(&input, &cli) {
                    Some(x) => println!("{}", x),
                    None => {}
                };
            }
            Err(error) => {
                eprintln!("error while reading line : {}", error);
                exit(1);
            }
        }
    }
}

fn clean_line<'a>(line: &'a str, cli: &Cli) -> Option<&'a str> {
    let mut index_of_first_bracket = None;
    let mut index_of_last_bracket = None;

    for (i, c) in line.chars().enumerate() {
        if c == '{' && index_of_first_bracket.is_none() {
            index_of_first_bracket = Some(i);
        } else if c == '}' {
            index_of_last_bracket = Some(i);
        }
    }

    match (cli, index_of_first_bracket, index_of_last_bracket) {
        (Cli { preserve_start: false, preserve_end: false }, Some(first), Some(last)) => Some(&line[first..(last + 1)]),
        (Cli { preserve_start: false, preserve_end: true }, Some(first), _) => Some(&line[first..]),
        (Cli { preserve_start: false, .. }, Some(first), None) => Some(&line[first..]),
        (Cli { preserve_start: true, .. }, _, Some(last)) => Some(&line[..(last + 1)]),
        (Cli { preserve_start: true, .. }, _, _) => Some(&line),
        (_, _, _) => None
    }
}

#[cfg(test)]
mod tests {
    use crate::{clean_line, Cli};

    #[test]
    fn no_json_no_preserve() {
        let cli = Cli {
            preserve_start: false,
            preserve_end: false,
        };
        assert_eq!(clean_line(&"aaaaa", &cli), None)
    }

    #[test]
    fn no_json_preserve_start() {
        let cli = Cli {
            preserve_start: true,
            preserve_end: false,
        };
        assert_eq!(clean_line(&"aaaaa", &cli), Some("aaaaa"))
    }

    #[test]
    fn no_json_preserve_end() {
        let cli = Cli {
            preserve_start: false,
            preserve_end: true,
        };
        assert_eq!(clean_line(&"aaaaa", &cli), None)
    }

    #[test]
    fn clean_start() {
        let cli = Cli {
            preserve_start: false,
            preserve_end: false,
        };
        assert_eq!(clean_line(&"aaaaa {\"count\": 0}", &cli), Some("{\"count\": 0}"))
    }

    #[test]
    fn clean_end() {
        let cli = Cli {
            preserve_start: false,
            preserve_end: false,
        };
        assert_eq!(clean_line(&"{\"count\": 0} aaaaaaaa", &cli), Some("{\"count\": 0}"))
    }

    #[test]
    fn clean_both() {
        let cli = Cli {
            preserve_start: false,
            preserve_end: false,
        };
        assert_eq!(clean_line(&"aaaaaaa {\"count\": 0} aaaaaaaa", &cli), Some("{\"count\": 0}"))
    }

    #[test]
    fn preserve_start() {
        let cli = Cli {
            preserve_start: true,
            preserve_end: false,
        };
        assert_eq!(clean_line(&"aaaaaaa {\"count\": 0} aaaaaaaa", &cli), Some("aaaaaaa {\"count\": 0}"))
    }

    #[test]
    fn preserve_end() {
        let cli = Cli {
            preserve_start: false,
            preserve_end: true,
        };
        assert_eq!(clean_line(&"aaaaaaaaaaa {\"count\": 0} aaaaaaaa", &cli), Some("{\"count\": 0} aaaaaaaa"))
    }
}
