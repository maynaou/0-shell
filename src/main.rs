use std::io::{self, Write};
mod commands;
<<<<<<< HEAD
<<<<<<< HEAD
mod parser;
use crate::commands::{cat, cd, cp, echo, ls, mkdir, mv, pwd, rm};
use fork::{Fork, fork};
fn main() {
    unsafe {
        signal(2, signal_handler);
=======
=======
>>>>>>> parent of 8fc6def (update cp vr finale)
use crate::commands::{echo,mkdir,ls,cat,cd}; 
use std::collections::HashMap;
fn main() {
    while let Some(cmd) = read_command() {
    let mut command = Vec::new();
    let line = cmd.split(" ").collect::<Vec<_>>().join(" "); 
    if let Some((cmd, rest)) = line.split_once(char::is_whitespace) {
        command.push(cmd.to_string());
        if rest != "" {
            command.push(rest.to_string()); 
        }
    } 
    if !command.is_empty(){
         match command[0].as_str() {
            "echo" => match command.len() > 1 {
                true => echo::echo(&command[1].trim()),
                false => echo::echo("")   
            },
            "mkdir" => match command.len() > 1 {
                true => mkdir::mkdir(&command[1].trim()),
                false => println!("mkdir: missing operand")
            } 
            "ls" => {
                let result = ls::ls(command[1..].to_vec());
                parsels(result);
            },
            "cat" => match command.len() > 1 {
                true => cat::cat(&command[1].trim()),
                false => cat::cat(""),
            },
            "cd" => match command.len() > 1 {
                true => cd::cd(&command[1].trim()),
                false => cd::cd(""),
            }
            "exit" => break,
             _ => match !command[0].is_empty() {
                true => println!("{}: not found",command[0]),
                false => continue,
            },  
          }
       }
    }  
}

fn parsels(mut result : (HashMap<String, Vec<String>>, bool)){
    if !result.1 {
        println!("Invalid flag found");
        return
>>>>>>> parent of 8fc6def (update cp vr finale)
    }
    
    while let Some(cmd) = parser::read_command() {
        let mut command = Vec::new();
        let line = cmd.split(" ").collect::<Vec<_>>().join(" ");
        if let Some((cmd, rest)) = line.split_once(char::is_whitespace) {
            command.push(cmd.to_string());
            if rest != "" {
                command.push(rest.to_string());
            }
        }

        if !command.is_empty() {
            match command[0].as_str() {
                // Commandes qui ne doivent PAS être forkées
                "cd" => match command.len() > 1 {
                    true => cd::cd(&command[1].trim()),
                    false => cd::cd(""),
                },
                "exit" => break,

                // Toutes les autres commandes sont forkées
                _ => {
                    match fork() {
                        Ok(Fork::Child) => {
                            unsafe {
                                signal(2, signal_handler_exit);
                            }
                            // Processus enfant - exécuter la commande
                            match command[0].as_str() {
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
                                    false => eprintln!("pwd: too many arguments"),
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
                                _ => match !command[0].is_empty() {
                                    true => println!("{}: not found", command[0]),
                                    false => {}
                                },
                            }
                            // Terminer le processus enfant
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
