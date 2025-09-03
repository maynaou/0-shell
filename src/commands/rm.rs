use std::{fs, fs::metadata, io, io::Write, path::Path};

pub fn rm(args: &str) {
    // println!("{}",args);

    let vec: Vec<&str> = args.split_whitespace().collect();
    // println!("{:?}",vec);
    let mut b = false;

    if vec.contains(&"-r") {
        b = true;
    }
    for i in 0..vec.len() {
        if vec[i] == "-r" {
            continue;
        }

        let path = match Path::new(vec[i]).exists() {
            true => match metadata(vec[i]) {
                Ok(n) => n,
                Err(e) => {
                    eprintln!("rm: cannot access '{}': {}", vec[i], e);
                    continue;
                }
            },
            false => match fs::symlink_metadata(vec[i]) {
                Ok(n) => n,
                Err(e) => {
                    eprintln!("rm: cannot access '{}': {}", vec[i], e);
                    continue;
                }
            },
        };
        match path.is_file() || path.is_symlink() {
            true => match path.permissions().readonly() {
                true => {
                    let mut input = String::new();
                    loop {
                        print!(
                            "rm: remove write-protected regular empty file '{}'? ",
                            vec[i]
                        );
                        if io::stdout().flush().is_err() {
                            break;
                        }

                        if io::stdin().read_line(&mut input).is_err() {
                            eprintln!("Erreur lecture stdin");
                            break;
                        }

                        if input[0..1].to_ascii_lowercase() == "y" {
                            match fs::remove_file(vec[i]) {
                                Ok(_) => {}
                                Err(e) => eprintln!("rm: cannot remove '{}': {}", vec[i], e),
                            }
                            break;
                        }else {
                            break;
                        }
                    }
                }
                false => match fs::remove_file(vec[i]) {
                    Ok(_) => {}
                    Err(e) => eprintln!("rm: cannot remove '{}': {}", vec[i], e),
                },
            },
            false => {
                if b {
                    match fs::remove_dir_all(vec[i]) {
                        Ok(_) => {}
                        Err(e) => eprintln!("rm: cannot remove '{}': {}", vec[i], e),
                    }
                } else {
                    eprintln!("rm: cannot remove '{}': Is a directory", vec[i]);
                }
            }
        }
    }
}
