use std::{fs, fs::metadata, io, path::Path};
#[derive(Debug)]
struct Format {
    count: usize,
    s: String,
}
pub fn cat(args: &str) {
    let mut vec  = Vec::new();
    if args.starts_with('\"') && args.ends_with('\"') || args.starts_with('\'') && args.ends_with('\'') {
        vec.push(&args[1..args.len()-1]);
    }else {
        let v_arg : Vec<&str> = args.split_whitespace().collect();
        vec.extend_from_slice(&v_arg);
    }

    if vec.len() == 0 {
        dash_empty();
    }
    let b = format_handle(vec.clone());
    if !b.s.is_empty() {
        if b.count == 1 {
            if let Some(first_char) = b.s.chars().find(|&c| c != '-') {
                println!("cat: invalid option -- '{}'", first_char);
            }
        } else {
            println!("cat: unrecognized option '{}'", b.s);
        }
    } else {
        for i in 0..vec.len() {
            if vec[i] == "-" {
                dash_empty();
            } else if vec[i] == "--" || vec[i] == "" {
                continue;
            } else {
                let content = match metadata(vec[i]) {
                    Ok(n) => match n.is_file() {
                        true => match Path::new(vec[i]).exists() {
                            true => match fs::read_to_string(vec[i]) {
                                Ok(n) => n,
                                Err(_) => "error".to_string(),
                            },
                            false => format!("cat: {}: No such file or directory", vec[i]),
                        },
                        false => format!("cat: {}: Is a directory", vec[i]),
                    },
                    Err(_) => format!("cat: {}: No such file or directory", vec[i]),
                };

                // println!("{}",content);
                // let temp;
                // if content.contains("\n") {
                //     temp = content.replace("\n", "'$'\\n''").replace("\"", "'");
                // }else if vec.len() == 1 {
                //     println!("----");
                //     temp = format!("cat: '{}': No such file or directory",vec[i]);
                // }else {
                //      println!("----");
                //     temp = content;
                // }
                println!("{}", content);
            }
        }
    }
}

fn format_handle(vec: Vec<&str>) -> Format {
    let mut count = 0;
    for i in 0..vec.len() {
        if vec[i].contains('-') && vec[i].len() > 2 {
            for c in vec[i].chars() {
                if c == '-' {
                    count += 1;
                } else {
                    break;
                }
            }

            if count > 0 {
                return Format {
                    count,
                    s: vec[i].to_string(),
                };
            }
        }
    }
    return Format {
        count: 0,
        s: Default::default(),
    };
}

fn dash_empty() {
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                break;
            }
            Ok(_) => {
                print!("{}", input);
            }
            Err(e) => {
                eprintln!("cat: error reading the input: {}", e);
                continue;
            }
        }
    }
}
