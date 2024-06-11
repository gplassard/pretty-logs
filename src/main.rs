mod cli;

use cli::Cli;
use env_logger::Builder;
use log::debug;
use log::error;
use std::io;
use std::process::exit;
use structopt::StructOpt;

fn main() {
    let cli = Cli::from_args();
    Builder::new().filter_level(cli.log_level).init();

    if atty::is(atty::Stream::Stdin) {
        exit(0);
    }
    debug!("tty stdin detected, proceeding");
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n == 0 {
                    exit(0);
                }
                if let Some(x) = clean_line(&input, &cli) {
                    println!("{}", x)
                };
            }
            Err(error) => {
                error!("error while reading line : {}", error);
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
    debug!(
        "Detected brackets indexes : first {:?}, last {:?}",
        index_of_first_bracket, index_of_last_bracket
    );
    match index_of_first_bracket {
        Some(first) => debug!("First part (len {:?}) : \"{}\"", first, &line[..first]),
        None => debug!("No first part detected"),
    }
    match index_of_last_bracket {
        Some(last) => debug!(
            "Last part (len {:?}) : \"{}\"",
            line.len() - (last + 1),
            &line[(last + 1)..]
        ),
        None => debug!("No last part detected"),
    }

    match (cli, index_of_first_bracket, index_of_last_bracket) {
        (
            Cli {
                preserve_start: false,
                preserve_end: false,
                ..
            },
            Some(first),
            Some(last),
        ) => Some(&line[first..(last + 1)]),
        (
            Cli {
                preserve_start: false,
                preserve_end: true,
                ..
            },
            Some(first),
            _,
        ) => Some(&line[first..]),
        (
            Cli {
                preserve_start: false,
                ..
            },
            Some(first),
            None,
        ) => Some(&line[first..]),
        (
            Cli {
                preserve_start: true,
                ..
            },
            _,
            Some(last),
        ) => Some(&line[..(last + 1)]),
        (
            Cli {
                preserve_start: true,
                ..
            },
            _,
            _,
        ) => Some(line),
        (_, _, _) => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::{clean_line, Cli};
    use log::LevelFilter::Error;

    #[test]
    fn no_json_no_preserve() {
        let cli = Cli {
            preserve_start: false,
            preserve_end: false,
            log_level: Error,
        };
        assert_eq!(clean_line("aaaaa", &cli), None)
    }

    #[test]
    fn no_json_preserve_start() {
        let cli = Cli {
            preserve_start: true,
            preserve_end: false,
            log_level: Error,
        };
        assert_eq!(clean_line("aaaaa", &cli), Some("aaaaa"))
    }

    #[test]
    fn no_json_preserve_end() {
        let cli = Cli {
            preserve_start: false,
            preserve_end: true,
            log_level: Error,
        };
        assert_eq!(clean_line("aaaaa", &cli), None)
    }

    #[test]
    fn clean_start() {
        let cli = Cli {
            preserve_start: false,
            preserve_end: false,
            log_level: Error,
        };
        assert_eq!(
            clean_line("aaaaa {\"count\": 0}", &cli),
            Some("{\"count\": 0}")
        )
    }

    #[test]
    fn clean_end() {
        let cli = Cli {
            preserve_start: false,
            preserve_end: false,
            log_level: Error,
        };
        assert_eq!(
            clean_line("{\"count\": 0} aaaaaaaa", &cli),
            Some("{\"count\": 0}")
        )
    }

    #[test]
    fn clean_both() {
        let cli = Cli {
            preserve_start: false,
            preserve_end: false,
            log_level: Error,
        };
        assert_eq!(
            clean_line("aaaaaaa {\"count\": 0} aaaaaaaa", &cli),
            Some("{\"count\": 0}")
        )
    }

    #[test]
    fn preserve_start() {
        let cli = Cli {
            preserve_start: true,
            preserve_end: false,
            log_level: Error,
        };
        assert_eq!(
            clean_line("aaaaaaa {\"count\": 0} aaaaaaaa", &cli),
            Some("aaaaaaa {\"count\": 0}")
        )
    }

    #[test]
    fn preserve_end() {
        let cli = Cli {
            preserve_start: false,
            preserve_end: true,
            log_level: Error,
        };
        assert_eq!(
            clean_line("aaaaaaaaaaa {\"count\": 0} aaaaaaaa", &cli),
            Some("{\"count\": 0} aaaaaaaa")
        )
    }
}
