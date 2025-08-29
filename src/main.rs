use std::{io::{self, Write}};
mod commands;
use crate::commands::{echo,mkdir,ls,cat}; 
use std::collections::HashMap;
fn main() {
    while let Some(cmd) = read_command() {
    // print!("$ ");
    //println!("--{}",cmd);
    let mut command = Vec::new();
    let line = cmd.split(" ").collect::<Vec<_>>().join(" "); 
    if let Some((cmd, rest)) = line.split_once(char::is_whitespace) {
        command.push(cmd.to_string());
        if rest != "" {
            command.push(rest.to_string()); 
        }
    } 
    //println!("{:?}",command);
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
            "exit" => break,
             _ => match !command[0].is_empty() {
                true => println!("{}: not found",command[0]),
                false => continue,
            },  
          }
        }
    }  
}

fn parsels(mut result : HashMap<String, Vec<String>>){
    let result_len = result.len();
    let mut countresult = 0;
    for (key, files) in &mut result {
        let filen = files.len();
        let mut counter = 1 ;
        if result_len == 1 || key == "." {
            // files.sort();
            for file in files {
                if counter != filen {
                    println!("{}", file);
                    counter += 1
                }else{
                    print!("{}",file);
                }
            }
        }else{
            println!("{}:", key);
            // files.sort();
            for file in &mut *files {
                if counter != filen {
                    println!("{}", file);
                    counter += 1
                }else{
                    println!("{}",file);
                }
            }
        }
        countresult += 1;
        if countresult != result_len || key == "." || result_len == 1{
            print!("\n");
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
        }
        res_f.push(c);
       }
    }

    if quoit.is_none() {
        return Some(res_f);
    }else {
        print!("> ");
    }
  }
}