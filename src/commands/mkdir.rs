use std::{fs,path::Path};
pub fn mkdir(args :&str) {
    let mut vec  = Vec::new();
    if args.starts_with('\"') && args.ends_with('\"') || args.starts_with('\'') && args.ends_with('\'') {
        vec.push(&args[1..args.len()-1]);
    }else {
        let v_arg : Vec<&str> = args.split_whitespace().collect();
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