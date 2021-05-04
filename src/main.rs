use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let tail = &args[1..];
    let content: String = tail.join(" ");
    println!("{}", content.trim_start_matches(|c| c != '{'));
}
