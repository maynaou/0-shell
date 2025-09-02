use std::{env,path::Path,env::set_current_dir,fs::metadata};

pub fn cd(args :&str) {
   
    let mut vec  = Vec::new();
    if  args.starts_with('\"') && args.ends_with('\"') || args.starts_with('\'') && args.ends_with('\'') {
        vec.push(&args[1..args.len()-1]);
    }else {
        let v_arg : Vec<&str> = args.split_whitespace().collect();
        vec.extend_from_slice(&v_arg);
    }

    println!("{:?}",vec);
    for i in 0..vec.len() {

       let _ = match vec[i] {
            "~" | "" => {
                if let Ok(home) = env::var("HOME") {
                    if let Err(e) = set_current_dir(Path::new(&home)) {
                        eprintln!("cd: {}", e);
                    }
                }
            }
            "." => continue,
            ".." => {
                if let Ok(current) = env::current_dir() {
                     unsafe { env::set_var("OLDPWD", &current) };
                    if let Some(parent) = current.parent() {
                        if let Err(e) = set_current_dir(parent) {
                            eprintln!("cd: {}", e);
                        }
                    }
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
                match metadata(vec[i]) {
                    Ok(meta) => {
                        if meta.is_dir() {
                            if let Ok(pwd) = env::current_dir() {
                                unsafe { env::set_var("OLDPWD",pwd) };
                            }

                            if let Err(e) = set_current_dir(Path::new(vec[i])) {
                                eprintln!("cd: {}", e);
                            }
                        } else {
                            eprintln!("cd: can't cd to {}", vec[i]);
                        }
                    },
                    Err(_) => eprintln!("cd: can't cd to {}", vec[i]),
                }
            }

        };


    }
}