use std::{env};

pub fn pwd() {
    // unsafe { env::set_var("OLDPWD",pwd) }
    match env::var("PWD") {
        Ok(path) => println!("{}", path),
        Err(_) => match env::current_dir() {
              Ok(path) => println!("{}",path.display()),
              Err(_) => println!(".")
        }
    }
}