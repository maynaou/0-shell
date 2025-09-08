use std::{
    fs,
    fs::metadata,
    io::{self, Read, Write},
};
#[derive(Debug)]
pub struct Format {
    pub count: usize,
    pub s: String,
}
pub fn cat(args: &str) {
    let mut vec = Vec::new();
    let mut aq = false;
    if args.starts_with('\"') && args.ends_with('\"')
        || args.starts_with('\'') && args.ends_with('\'')
    {
        aq = true;
        vec.push(&args[1..args.len() - 1]);
    } else {
        let v_arg: Vec<&str> = args.split_whitespace().collect();
        vec.extend_from_slice(&v_arg);
    }

    if vec.len() == 0 || vec[0] == "--" && vec.len() == 1 {
        dash_empty();
        return;
    }

    let b = format_handle(vec.clone());
    if !b.s.is_empty() {
        if b.count < 2 {
            if let Some(first_char) = b.s.chars().find(|&c| c != '-')
                && b.s.starts_with('-')
            {
                println!("cat: invalid option -- '{}'", first_char);
            } else {
                for i in 0..vec.len() {
                    if vec[i] == "-" {
                        dash_empty();
                    } else if vec[i] == "" || vec[i] == "--" {
                        continue;
                    } else {
                        let content = match metadata(vec[i]) {
                            Ok(n) => match n.is_file() {
                                true => match fs::read_to_string(vec[i]) {
                                    Ok(n) => n,
                                    Err(_) => "error".to_string(),
                                },
                                false => format!("cat: {}: Is a directory", vec[i]),
                            },
                            Err(_) => {
                                format!("cat: {}: No such file or directory", vec[i])
                            }
                        };

                        let v: Vec<&str> = vec[i].split_whitespace().collect();
                        let mut temp = content;
                        if aq && v.len() > 1 {
                            temp = format!(
                                "cat: '{}': No such file or directory",
                                vec[i].replace("\n", "'$'\\n''")
                            );
                        }

                        println!("{}", temp);
                    }
                }
            }
        } else {
            println!("cat: unrecognized option '{}'", b.s);
        }
    }
}

pub fn format_handle(vec: Vec<&str>) -> Format {
    println!("{:?}", vec);
    let mut count = 0;
    for i in 0..vec.len() {
        for c in vec[i].chars() {
            if c == '-' {
                count += 1;
            } else {
                break;
            }
        }

        if count > 0 && vec[i] != "-r" {
            if (vec[i] != "-" && vec[i] != "--") || vec.len() < 3 {
                return Format {
                    count,
                    s: vec[i].to_string(),
                };
            }
        } else {
            count = 0;
        }
    }
    return Format {
        count: 0,
        s: Default::default(),
    };
}

fn dash_empty() {
    let mut stdin = io::stdin();
    let mut buffer = [0; 1]; // Buffer comme dans le C original

    loop {
        match stdin.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                if io::stdout().write(&buffer[..n]).is_err() {
                    eprintln!("cat: write error");
                }
                let _ = io::stdout().flush();
            }
            Err(e) => {
                eprintln!("cat: read error: {}", e);
            }
        }
    }
}
