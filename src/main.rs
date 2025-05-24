use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{self, Command};

fn exit(code: &str) {
    let exit_status = code.parse().expect("failed on exit status");
    process::exit(exit_status)
}

fn echo(args: &[&str]) {
    for arg in args {
        print!("{arg} ");
    }
    println!();
}

fn pathfind(command: &str) -> Option<PathBuf> {
    let path = env::var_os("PATH")?;
    for dir in env::split_paths(&path) {
        let target = dir.join(command);
        if let Ok(metadata) = fs::metadata(&target) {
            if metadata.is_file() {
                return Some(target);
            }
        }
    }
    None
}

fn r#type(command: &str) {
    match command {
        "exit" | "echo" | "type" => {
            println!("{} is a shell builtin", command);
            return;
        }
        _ => {}
    }

    if let Some(path) = pathfind(command) {
        println!("{} is {}", command, path.display());
    } else {
        println!("{}: not found", command);
    }
}

fn runprogram(path: &Path, args: &[&str]) {
    let mut program = Command::new(path.file_name().expect("failed on filename"));
    if !args.is_empty() {
        program.args(args);
    }
    program
        .spawn()
        .expect("failed to spawn")
        .wait()
        .expect("failed on wait");
}

fn main() {
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
            "exit" => exit(args[0]),
            "echo" => echo(args),
            "type" => r#type(args[0]),
            _ => match pathfind(command) {
                Some(path) => runprogram(&path, args),
                None => println!("{cmd}: command not found"),
            },
        }
    }
}
