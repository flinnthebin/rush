#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {
    let exit_status: i32;
    loop {
        // prompt
        print!("$ ");
        io::stdout().flush().unwrap();

        // user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let cmd = input.trim();
        let v: Vec<&str> = input.split_whitespace().collect();
        let command = v[0];
        let args: &[&str] = &v[1..];

        // command interpretation
        match command {
            "exit" => {
                exit_status = v[1].parse().expect("exit status not a number");
                break;
            }
            "echo" => {
                for arg in args {
                    print!("{arg} ");
                }
                println!();
            }
            _ => println!("{cmd}: command not found"),
        }
    }
    process::exit(exit_status)
}
