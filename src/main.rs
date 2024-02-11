use std::io::BufRead;
use std::{env, io};
use colored::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("use: {} <pattern>", args[0]);
        std::process::exit(1);
    }

    let patterns = &args[1..];

    let stdin = io::stdin();
    let reader = stdin.lock();

    for (x, l) in reader.lines().enumerate() {
        let line = l.unwrap();
        for p in patterns {
            if line.contains(p) {
                let line = line.trim();

                if let Some(index) = line.find(p) {
                    let before = &line[..index];
                    let match_text = &line[index..index + p.len()];

                    let after = &line[index + p.len()..];

                    let result = format!("line: {}, string: {}{}{}", x, before, match_text.red(),
                                         after);
                    println!("{}", result);
                }
            }
        }
    }
}
