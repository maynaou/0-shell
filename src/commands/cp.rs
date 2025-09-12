use super::cat;
use std::fs;
use std::path::Path;

pub fn cp(args: &str) {
    let mut vec = Vec::new();
    if args.starts_with('\"') && args.ends_with('\"')
        || args.starts_with('\'') && args.ends_with('\'')
    {
        vec.push(&args[1..args.len() - 1]);
    } else {
        let v_arg: Vec<&str> = args.split_whitespace().collect();
        vec.extend_from_slice(&v_arg);
    }

    let destination = vec[vec.len() - 1]; //[a,b,c]  ---> c | [a,b] --> b
    let sources = &vec[0..vec.len() - 1]; // [a,b]   ---> [a]

    let dest_is_dir = Path::new(destination).is_dir(); // true : directory false file or tkhanticha

    if sources.len() > 1 && !dest_is_dir  {
        eprintln!("cp: target '{}' is not a directory", destination);
        return;
    }
    let b = cat::format_handle(vec.clone(), "cat");
    if !b.s.is_empty() {
        if b.count < 2 {
            if let Some(first_char) = b.s.chars().find(|&c| c != '-')
                && b.s.starts_with('-')
            {
                println!("cat: invalid option -- '{}'", first_char);
            } else if !b.s[b.count..].is_empty() && vec.len() == 1 || b.s == "-" {
                println!("cp: missing destination file operand after '{}'",b.s);
            } else {
                for source in sources {
                    let final_dest = if dest_is_dir {
                        format!("{}/{}", destination, source)
                    } else {
                        destination.to_string()
                    };

                    match fs::copy(source, &final_dest) {
                        Ok(_) => (),
                        Err(_) => {
                            eprintln!("cp: cannot stat '{}': No such file or directory", source);
                        }
                    }
                }
            }
        } else {
            println!("cat: unrecognized option '{}'", b.s);
        }
    } else {
        println!("cp: missing file operan");
    }
}
