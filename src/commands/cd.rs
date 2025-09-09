use super::cat;
use std::{env,path::Path,env::set_current_dir,fs::metadata};

pub fn cd(args :&str) {
    let mut vec  = Vec::new();
    if  args.starts_with('\"') && args.ends_with('\"') || args.starts_with('\'') && args.ends_with('\'') {
        vec.push(&args[1..args.len()-1]);
    }else {
        let v_arg : Vec<&str> = args.split_whitespace().collect();
        vec.extend_from_slice(&v_arg);
    }
    let b = cat::format_handle(vec.clone(), "cd");
       if !b.s.is_empty() {
        if b.count < 2 {
            if let Some(first_char) = b.s.chars().find(|&c| c != '-')
                && b.s.starts_with('-')
            {
                println!("cd: Illegal option '{}'", first_char);
            } else {
       match vec[0] {
            "~" | "" => {
                if let Ok(pwd) = env::current_dir() {
                    unsafe { env::set_var("OLDPWD",pwd) };
                }

                if let Ok(home) = env::var("HOME") {
                    if let Err(e) = set_current_dir(Path::new(&home)) {
                        eprintln!("cd: {}", e);
                    }
                }

                if let Ok(pwd) = env::current_dir() {
                    unsafe { env::set_var("PWD",pwd) };
                }
            }
            "." => print!(""),
            ".." => {
                match env::current_dir() {
                    Ok(current) =>  {
                     unsafe { env::set_var("OLDPWD", &current) };
                     if let Some(parent) = current.parent() {
                        if let Err(e) = set_current_dir(parent) {
                            eprintln!("cd: {}", e);
                        }
                     }
                     },
                    Err(_) => eprintln!("cd: can't cd to .."),
                }
            }

            "-" => {
                if let Ok(oldpwd) = env::var("OLDPWD") {
                        if let Ok(pwd) = env::current_dir() {
                            unsafe { env::set_var("OLDPWD",pwd) };
                        }
                    if let Err(e) = set_current_dir(&oldpwd) {
                        eprintln!("cd: {}", e);
                    } else {
                        println!("{}", oldpwd);
                    }
                } else {
                    eprintln!("cd: OLDPWD not set");
                }
            }
            _ => {
                match metadata(vec[0]) {
                    Ok(meta) => {
                        if meta.is_dir() {
                            if let Ok(pwd) = env::current_dir() {
                                unsafe { env::set_var("OLDPWD",pwd) };
                            }

                            if let Err(e) = set_current_dir(Path::new(vec[0])) {
                                eprintln!("cd: {}", e);
                            }
                            if let Ok(pwd) = env::current_dir() {
                                unsafe { env::set_var("PWD",pwd) };
                            }
                           
                        } else {
                            eprintln!("cd: can't cd to {}", vec[0]);
                        }
                    },
                    Err(_) => {
                         eprintln!("cd: can't cd to {}", vec[0])
                    },
                }
            }
    }
     }
        } else {
            println!("cd: Illegal option --");
        }
    }
}