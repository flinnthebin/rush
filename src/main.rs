use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{self, Command};

fn exit(args: &[&str]) {
    let mut exit_status: i32 = args[0].parse().expect("exit argument not an int");
    if args.is_empty() {
        exit_status = 0
    }
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

fn r#type(args: &[&str]) {
    let command = args[0];
    match command {
        "exit" | "echo" | "type" | "pwd" => {
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

fn runprogram(path: &Path, args: &[&str]) -> io::Result<()> {
    let mut command = Command::new(path.file_name().expect("File name error"));
    command.args(args);
    _ = command.spawn()?.wait()?;
    Ok(())
}

fn pwd() -> io::Result<()> {
    let curr_dir = env::current_dir()?;
    println!("{}", curr_dir.display());
    Ok(())
}
fn cd(args: &[&str]) -> io::Result<()> {
    let mut target = match args.first() {
        None | Some(&"~") => env::var_os("HOME").expect("Home dir error").into(),
        Some(s) if s.starts_with("~/") => {
            let mut home: PathBuf = env::var_os("HOME").expect("Home dir error 2").into();
            home.push(&s[2..]);
            home
        }
        Some(s) => PathBuf::from(s),
    };

    if !target.exists() {
        eprintln!("cd: {}: No such file or directory", target.display());
    } else if !target.is_dir() {
        eprintln!("cd: {}: Not a directory", target.display());
    } else {
        env::set_current_dir(&target)?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
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
            "exit" => exit(args),
            "echo" => echo(args),
            "type" => r#type(args),
            "pwd" => pwd()?,
            "cd" => cd(args)?,
            _ => match pathfind(command) {
                Some(path) => runprogram(&path, args)?,
                None => println!("{cmd}: command not found"),
            },
        }
    }
}
