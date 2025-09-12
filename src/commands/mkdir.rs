use shell::*;

use super::{cat};
use std::{fs, path::Path};

pub fn mkdir(args: &str) {
    let temp = new_ligne(args, false);

    let mut vec = Vec::new();
    if temp.starts_with('\"') && temp.ends_with('\"')
        || temp.starts_with('\'') && temp.ends_with('\'')
    {
        vec.push(&temp[1..temp.len() - 1]);
    } else {
        let v_arg: Vec<&str> = temp.split_whitespace().collect();
        vec.extend_from_slice(&v_arg);
    }

    let b = cat::format_handle(vec.clone(),"mkdir");

    if !b.s.is_empty() {
       if b.count < 2 {
            if let Some(first_char) = b.s.chars().find(|&c| c != '-') && b.s.starts_with('-') {
                println!("mkdir: invalid option -- '{}'", first_char);
            } else {
                for i in 0..vec.len() {
                    if vec[i] == "--" {
                          continue;
                    }
                    match Path::new(vec[i]).exists() {
                        true => {
                            println!("mkdir: cannot create directory ‘{}’: File exists", vec[i])
                        }
                        false => match fs::create_dir(vec[i]) {
                            Ok(n) => n,
                            Err(_) => (),
                        },
                    }
                }
            }
        } else {
            println!("mkdir: unrecognized option '{}'", b.s);
        }
    }else {
         println!("mkdir: missing operand");
    }
   
}
