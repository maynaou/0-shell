use std::{io::{self, Write}};
mod commands;
use crate::commands::echo; 
fn main() {
    while let Some(cmd) = read_command() {
    // print!("$ ");

    let mut command = Vec::new();
    let line = cmd.split(" ").collect::<Vec<_>>().join(" "); 

    if let Some((cmd, rest)) = line.split_once(char::is_whitespace) {
        command.push(cmd); 
        command.push(rest); 
    } else {
        command.push(&line);
    }

    if !command.is_empty(){
         match command[0] {
            "echo" => match command.len() > 1 {
                true => echo::echo(&command[1].trim()),
                false => echo::echo("")   
            }
            "exit" => break,
             _ => println!("command not found"),  
          }
        }

    }
      
}

fn read_command() -> Option<String> {
let mut input = String::new();
let mut quoit:Option<char>;
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

    if io::stdin().read_line(&mut input).is_err() {
        eprintln!("Erreur lecture stdin");
        return None;  
    }

    vec.push(input.to_string());
    for i in 0..vec.len() {
     for c in vec[i].chars() {
        if c == '\'' || c == '\"' {
            if let Some(q) = quoit {
                 if c == q {
                    quoit = None;
                 }
            }else {
                quoit = Some(c);
            }
        }else {
             res_f.push(c);
         }
       }
    }

    if quoit.is_none() {
        return Some(res_f);
    }else {
        print!("> ");
    }
  }
}