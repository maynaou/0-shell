use std::io::{self,Write};
use std::process::Command;

fn execute_backticks(cmd: &str) -> String {
    let trimmed = cmd.trim_matches('`');
    let output = Command::new(trimmed).output();
    match output {
        Ok(o) => String::from_utf8_lossy(&o.stdout).trim().to_string(),
        Err(_) => format!("{}: command not found", trimmed),
    }
}

pub fn echo(args: &[&str]) {
    let mut result = Vec::new();
    for arg in args {
        if arg.starts_with('`') && arg.ends_with('`') {
            result.push(execute_backticks(arg));
        } else {
            result.push(arg.to_string());
        }
    }
    println!("{}", result.join(" "));
}
