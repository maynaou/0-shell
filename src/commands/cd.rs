use std::{env,path::Path,env::set_current_dir};

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
        if vec[i] == "." {
            continue;
        }else if vec[i] == "~" {
            let s = match env::var("HOME") {
                Ok(n) => n,
                Err(_) => todo!(),
            };
            
            let _ = set_current_dir(Path::new(&s)) ;
                     
        }
    }
}