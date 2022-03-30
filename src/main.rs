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
                match clean_line(&input) {
                    Some(x) => println!("{}", x),
                    None => {}
                };
            }
            Err(error) => {
                eprintln!("error while reading line : {}", error);
                exit(1);
            },
        }
    }
}

fn clean_line(line: &String) -> Option<&str> {
    if line.contains('{') {
        Some(line.trim_start_matches(|c| c != '{'))
    }
    else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::clean_line;

    #[test]
    fn non_matching_string() {
        assert_eq!(clean_line(&"aaaaa".to_string()), None)
    }

    #[test]
    fn matching_string() {
        assert_eq!(clean_line(&"aaaaa {\"count\": 0}".to_string()), Some("{\"count\": 0}"))
    }
}
