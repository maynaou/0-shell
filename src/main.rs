use std::io::{self, Write};
mod commands;
use crate::commands::{cat, cd, cp, echo, ls, mkdir, mv, pwd, rm};
use fork::{Fork, fork};
use std::collections::HashMap;
fn main() {
    unsafe {
        signal(2, signal_handler);
    }
    
    while let Some(cmd) = read_command() {
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
                                    parsels(result);
                                }
                                "cat" => match command.len() > 1 {
                                    true => cat::cat(&command[1].trim()),
                                    false => cat::cat(""),
                                },
                                "pwd" => match command.len() == 1 {
                                    true => pwd::pwd(),
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

fn parsels(mut result: (HashMap<String, Vec<String>>, bool)) {
    if !result.1 {
        println!("Invalid flag found");
        return;
    }
    let result_len = result.0.len();
    let mut countresult = 0;
    for (key, files) in &mut result.0 {
        let filen = files.len();
        let mut counter = 1;
        if result_len == 1 || key == "." {
            // files.sort();
            for file in files {
                if counter != filen {
                    println!("{}", file);
                    counter += 1
                } else {
                    print!("{}", file);
                }
            }
        } else {
            println!("{}:", key);
            // files.sort();
            for file in &mut *files {
                if counter != filen {
                    println!("{}", file);
                    counter += 1
                } else {
                    println!("{}", file);
                }
            }
        }
        countresult += 1;
        if countresult != result_len || key == "." || result_len == 1 {
            print!("\n");
        }
    }
}

fn read_command() -> Option<String> {
    let mut input = String::new();
    let mut quoit: Option<char>;
    let mut res_f = String::new();
    let mut vec = vec![];
    loop {
        input.clear();
        res_f.clear();
        quoit = None;
        if vec.len() == 0 {
            print!("$ ");
        }

        if io::stdout().flush().is_err() {
            return None;
        }

        match io::stdin().read_line(&mut input) {
            Ok(0) => std::process::exit(0),
            Ok(_) => {}
            Err(_) => eprintln!("Error lecture"),
        }

        vec.push(input.to_string());

        for i in 0..vec.len() {
            for c in vec[i].chars() {
                if c == '\'' || c == '\"' {
                    if let Some(q) = quoit {
                        if c == q {
                            quoit = None;
                        }
                    } else {
                        quoit = Some(c);
                    }
                }
                res_f.push(c);
            }
        }

        if quoit.is_none() {
            return Some(res_f);
        } else {
            print!("> ");
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
