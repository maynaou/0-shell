use std::io::{self, Write};
mod commands;
use crate::commands::echo; 
fn main() {
    loop {
        let mut s = String::new();

        print!("$ ");
        match io::stdout().flush() {
            Ok(_) => {}
            Err(e) if e.kind() == io::ErrorKind::BrokenPipe => {
                // pipe fermé → on termine proprement
                return;
            }
            Err(e) => {
                eprintln!("Erreur flush stdout: {}", e);
                return;
            }
        }

        if io::stdin().read_line(&mut s).is_err() {
            eprintln!("Erreur lecture stdin");
            break;
        }

        let command:Vec<&str> = s.split_whitespace().collect();
        match command[0] {
            "echo" => echo::echo(&command[1..]),
            _ => todo!()  
        }
        println!("command : {:?}",command);
    }
}
