use shell::*;
use std::fs;
use std::io;
use std::path::Path;

pub fn mv(args: &str) {
    let  vec:  Vec<&str> = args.split_whitespace().collect();

    let b = format_handle(vec.clone(), "mv");
    if !b.s.is_empty() {
        if b.count < 2 {
            if let Some(first_char) = b.s.chars().find(|&c| c != '-')
                && b.s.starts_with('-')
            {
                println!("cp: invalid option -- '{}'", first_char);
            } else if !b.s[b.count..].is_empty() && vec.len() == 1 || b.s == "-" {
                println!("cp: missing destination file operand after '{}'", b.s);
            } else {
                let destination = vec[vec.len() - 1];
                let sources = &vec[0..vec.len() - 1];

                let dest_is_dir = Path::new(destination).is_dir();

                if sources.len() > 1 && !dest_is_dir {
                    eprintln!("mv: target '{}' is not a directory", destination);
                    return;
                }

                if sources.len() == 1
                    && Path::new(sources[0]).is_dir()
                    && !dest_is_dir
                {
                    eprintln!(
                        "mv: cannot overwrite non-directory '{}' with directory '{}'",
                        destination, sources[0]
                    );
                    return;
                }
                for source in sources {
                    let final_dest = if dest_is_dir {
                        format!("{}/{}", destination, source)
                    } else {
                        destination.to_string()
                    };

                    match fs::rename(source, &final_dest) {
                        Ok(_) => (),
                        Err(e) => {
                            if e.kind() == io::ErrorKind::NotFound {
                                eprintln!(
                                    "mv: cannot stat '{}': No such file or directory",
                                    source
                                );
                            } else {
                                eprintln!(
                                    "mv: cannot move '{}' to '{}': {}",
                                    source, final_dest, e
                                );
                            }
                        }
                    }
                }
            }
        } else {
            println!("cat: unrecognized option '{}'", b.s);
        }
    } else {
        println!("cp: missing file operand");
    }
}
