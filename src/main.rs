use std::io::{self, Write};
mod commands;
mod parser;
use crate::commands::*;
use fork::{Fork, fork};
use shell::handle_quoit;
fn main() {
    unsafe {
        signal(2, signal_handler);
    }

    while let Some(cmd) = parser::read_command() {
        let mut command = Vec::new();
        let line: String;

        if let Some((cmd, rest)) = cmd.split_once(' ') {
            command.push(cmd.to_string());

            if rest != "" {
                if rest.starts_with('\"') && rest.ends_with('\"')
                    || rest.starts_with('\'') && rest.ends_with('\'')
                {
                    line = rest.split(" ").collect::<Vec<_>>().join(" ");
                } else {
                    line = rest.split_whitespace().collect::<Vec<_>>().join(" ");
                }
                command.push(line.to_string());
            }
        } else {
            command.push(cmd);
        }

        if !command.is_empty() {
            match command[0].trim_matches(|c| c == '"' || c == '\'') {
                "cd" => match command.len() > 1 {
                    true => cd::cd(&command[1].trim()),
                    false => cd::cd(""),
                },
                "exit" => break,
                "" => continue,
                _ => {
                    match fork() {
                        Ok(Fork::Child) => {
                            unsafe {
                                signal(2, signal_handler_exit);
                            }
                            match command[0].trim_matches(|c| c == '"' || c == '\'') {
                                "echo" => match command.len() > 1 {
                                    true => echo::echo(&command[1].trim()),
                                    false => echo::echo(""),
                                },
                                "mkdir" => match command.len() > 1 {
                                    true => mkdir::mkdir(&command[1].trim()),
                                    false => println!("mkdir: missing operand"),
                                },
                                "ls" => {
                                    let result = ls::ls(command[1..].to_vec());
                                    parser::parsels(result);
                                }
                                "cat" => match command.len() > 1 {
                                    true => cat::cat(&command[1].trim()),
                                    false => cat::cat(""),
                                },
                                "pwd" => match command.len() > 1 {
                                    true => pwd::pwd(&command[1].trim()),
                                    false => pwd::pwd(""),
                                },
                                "cp" => match command.len() > 1 {
                                    true => cp::cp(&command[1].trim()),
                                    false => eprintln!("cp: missing file operand"),
                                },
                                "mv" => match command.len() > 1 {
                                    true => mv::mv(&command[1].trim()),
                                    false => eprintln!("mv: missing file operand"),
                                },
                                "rm" => rm::rm(&command[1].trim()),
                                _ => {
                                    let temp = handle_quoit(command[0].to_string());
                                    println!("{}: not found", temp);
                                }
                            }
                            std::process::exit(0);
                        }
                        Ok(Fork::Parent(ch)) => {
                            unsafe {
                                signal(2, signal_handler_ln);
                            }
                            match fork::waitpid(ch) {
                                Ok(_) => {}
                                Err(_) => eprintln!("faild to wait child"),
                            }
                            unsafe {
                                signal(2, signal_handler);
                            }
                        }
                        Err(_) => {
                            // Erreur de fork
                            eprintln!("fork failed");
                        }
                    }
                }
            }
        }
    }
}

extern "C" fn signal_handler(_n: i32) {
    print!("\n$ ");
    if io::stdout().flush().is_err() {}
}
extern "C" fn signal_handler_ln(_n: i32) {
    println!();
}
extern "C" fn signal_handler_exit(_n: i32) {
    // println!("exit ====>");
    std::process::exit(0)
}
unsafe extern "C" {
    fn signal(signal: i32, handler: extern "C" fn(i32));
}
