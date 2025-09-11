use std::io::{self,Write};
use std::collections::HashMap;
pub fn read_command() -> Option<String> {
    let mut input = String::new();
    let mut quoit: Option<char>;
    let mut res_f = String::new();
    let mut vec = vec![];
    loop {
        input.clear();
        res_f.clear();
        quoit = None;
        if vec.len() == 0 {
            print!("$ ");
        }

        if io::stdout().flush().is_err() {
            return None;
        }

        match io::stdin().read_line(&mut input) {
            Ok(0) => std::process::exit(0),
            Ok(_) => {}
            Err(_) => eprintln!("Error lecture"),
        }
        
        vec.push(input.to_string());

        for i in 0..vec.len() {
            for c in vec[i].chars() {
                if c == '\'' || c == '\"' {
                    if let Some(q) = quoit {
                        if c == q {
                            quoit = None;
                        }
                    } else {
                        quoit = Some(c);
                    }
                }
                res_f.push(c);
            }
        }

        if quoit.is_none() {
            // print!("sssssssssss{}",&res_f);
            return Some(res_f.trim().to_string());
        } else {
            print!("> ");
        }
    }
}


pub fn parsels(mut result: (HashMap<String, Vec<String>>, bool)) {
    if !result.1 {
        println!("Invalid flag found");
        return;
    }
    let result_len = result.0.len();
    let mut countresult = 0;
    for (key, files) in &mut result.0 {
        let filen = files.len();
        let mut counter = 1;
        if result_len == 1 || key == "." {
            // files.sort();
            for file in files {
                if counter != filen {
                    println!("{}", file);
                    counter += 1
                } else {
                    print!("{}", file);
                }
            }
        } else {
            println!("{}:", key);
            // files.sort();
            for file in &mut *files {
                if counter != filen {
                    println!("{}", file);
                    counter += 1
                } else {
                    println!("{}", file);
                }
            }
        }
        countresult += 1;
        if countresult != result_len || key == "." || result_len == 1 {
            print!("\n");
        }
    }
}
