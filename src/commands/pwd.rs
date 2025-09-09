use super::cat;
use std::env;

pub fn pwd(args: &str) {
    // unsafe { env::set_var("OLDPWD",pwd) }
    let mut vec = Vec::new();
    // let mut aq = false;
    if args.starts_with('\"') && args.ends_with('\"')
        || args.starts_with('\'') && args.ends_with('\'')
    {
        // aq = true;
        vec.push(&args[1..args.len() - 1]);
    } else {
        let v_arg: Vec<&str> = args.split_whitespace().collect();
        vec.extend_from_slice(&v_arg);
    }

    let b = cat::format_handle(vec.clone(), "pwd");
    if !b.s.is_empty() {
        if b.count < 3 {
            if let Some(first_char) = b.s.chars().find(|&c| c != '-')
                && b.s.starts_with('-')
            {
                println!("pwd: Illegal option '{}'", first_char);
            } else {
                match env::var("PWD") {
                    Ok(path) => println!("{}", path),
                    Err(_) => match env::current_dir() {
                        Ok(path) => println!("{}", path.display()),
                        Err(_) => println!("."),
                    },
                }
            }
        } else {
            println!("pwd: Illegal option --");
        }
    } else {

    }
}
