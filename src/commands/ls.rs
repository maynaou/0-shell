use std::fs;
use std::collections::HashMap;
pub fn ls(flags : Vec<String>) -> HashMap<String, Vec<String>> {
    //println!("{:?}",flags);
    let validfomart = vformat(flags);
    let flaga = checkaf(&validfomart);
    let flagf = checkf(&validfomart);
    let mut files: HashMap<String, Vec<String>> = HashMap::new();
    let  path = getdirnames(&validfomart);
    //println!("{:?}",path);
    for arg in &path {
        files.insert(arg.to_string(), Vec::new());
           match fs::read_dir(&arg) {
            Ok(iterdir) => {
                for iter in iterdir {
                    let iter = iter.unwrap();
                    let filename_os = iter.file_name();
                    let filename = filename_os.to_string_lossy().into_owned(); 
                    if let Some(vec) = files.get_mut(arg){
                        if flaga {
                            vec.push(filename);
                        }else{
                            if !filename.starts_with('.') {
                                vec.push(filename);
                            }
                        }
                    }
                }
            },
            Err(_) => {
                if let Some(vec) = files.get_mut(arg){
                    vec.push(format!("ls: cannot access '{}': No such file or directory", arg));
                }
            }
        }
    }
    if !flagf {
        for (_, vec_files) in &mut files {
            vec_files.sort();
        }
    }
    files
}

fn getdirnames(flags : &Vec<String>) -> Vec<String> {
    let mut result : Vec<String> = Vec::new();
    let mut bol = false;
    for arg in flags {
        if !arg.starts_with("-") && arg != "" {
            bol = true;
            result.push(arg.clone());
        }
    }
    if  flags.len() > 0 && flags[0] == "" || !bol {
        result.push(".".to_string())
    }else {
        result.push(".".to_string())
    }

    result
}
fn checkf(flags : &Vec<String>) -> bool {
    for r in flags {
        if r.starts_with("-") {
            if  r.contains("f") {
                return true
            }
        }
    }
    false
}
fn checkaf(flags : &Vec<String>) -> bool {
    for r in flags {
        if r.starts_with("-") {
            if r.contains("a") || r.contains("f") {
                return true
            }
        }
    }
    false
}
fn vformat(flags : Vec<String>) -> Vec<String> {
    // let result : Vec<String> = Vec::new();
    if flags.len() > 0  {
          if  flags[0] == ""{
        return flags;
    }
    let parts: Vec<String> = flags[0]
    .split(' ')                // split by space
    .map(|s| s.trim())         // trim each substring
    .filter(|s| !s.is_empty()) // remove empty strings caused by multiple spaces
    .map(|s| s.to_string())    // convert &str to String
    .collect();
     return parts;
    }
   flags
}