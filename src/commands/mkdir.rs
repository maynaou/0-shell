use super::echo;
use std::{fs,path::Path};

pub fn mkdir(args :&str) {
    let temp = echo::new_ligne(args);
    let mut vec  = Vec::new();
    if  temp.starts_with('\"') && temp.ends_with('\"') || temp.starts_with('\'') && temp.ends_with('\'') {
        vec.push(&temp[1..temp.len()-1]);
    }else {
        let v_arg : Vec<&str> = temp.split_whitespace().collect();
        vec.extend_from_slice(&v_arg);
    }
    for i in 0..vec.len() { 
        match Path::new(vec[i]).exists() {
        true => println!("mkdir: cannot create directory ‘{}’: File exists",vec[i]),
        false => match fs::create_dir(vec[i]) {
            Ok(n) => n, 
            Err(_) => (),
        },
      }
    }
  
   /* match Path::new(args).exists() {
        true => println!("mkdir: cannot create directory ‘{}’: File exists",args),
        false => match fs::create_dir(args) {
            Ok(n) => n, 
            Err(_) => (),
        },
    }*/
}
