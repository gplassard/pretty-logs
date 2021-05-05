use std::io;
use std::process::exit;

fn main() {
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
                handle_line(input);
            }
            Err(error) => {
                eprintln!("error: {}", error);
                exit(1);
            },
        }
    }
}

fn handle_line(line: String) {
    if line.contains('{') {
        print!("{}", line.trim_start_matches(|c| c != '{'));
    }
    else {
        print!("{}", line);
    }
}
