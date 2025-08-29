use std::fs;
use std::collections::HashMap;
use std::os::unix::fs::PermissionsExt; // for executable
use std::os::unix::fs::FileTypeExt;    // for pipe and socket
use std::path::Path;
pub fn ls(flags : Vec<String>) -> (HashMap<String, Vec<String>>, bool) {
    //println!("{:?}",flags);
    let mut files: HashMap<String, Vec<String>> = HashMap::new();
    let validfomart = vformat(flags);
    let flagsvalid = validflags(&validfomart);
    if !flagsvalid {
        return (files,false)
    }
    let flaga = checkaf(&validfomart);
    let flagf = checkf(&validfomart);
    let  path = getdirnames(&validfomart);
    for arg in &path {
        files.insert(arg.to_string(), Vec::new());
           match fs::read_dir(&arg) {
            Ok(iterdir) => {
                for iter in iterdir {
                    let iter = iter.unwrap();
                    let typofiter = iter.file_type().unwrap();
                    let symbol = if typofiter.is_dir() {
                        "/"
                    } else if typofiter.is_symlink() {
                        "@"
                    } else if is_pipe(&iter.path()) {
                        "|"
                    } else if is_socket(&iter.path()) {
                        "="
                    } else if is_executable(&iter.path()) {
                        "*"
                    } else {
                        ""
                    };
                    
                    let filename_os = iter.file_name();
                    let mut filename = filename_os.to_string_lossy().into_owned(); 
                    if flagf  {
                        filename.push_str(symbol);

                    }
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
        for (_, vec_files) in &mut files {
            vec_files.sort();
        }
    (files, true)
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
fn validflags(flags : &Vec<String>) -> bool {
    let valid_flags = ['a', 'F', 'l'];
    for r in flags {
        if r.starts_with("-") {
            for ch in r.chars().skip(1) {
                if !valid_flags.contains(&ch) {
                    return false
                }
            }
        }
    }
    true
}
fn checkf(flags : &Vec<String>) -> bool {
    for r in flags {
        if r.starts_with("-") {
            if  r.contains("F") {
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
    let parts: Vec<String> = flags[0].split(' ').map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
     return parts;
    }
   flags
}

// check if a file is executable
fn is_executable(path: &Path) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        metadata.permissions().mode() & 0o111 != 0 // any execute bit set
    } else {
        false
    }
}

// check if file is named pipe
fn is_pipe(path: &Path) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        metadata.file_type().is_fifo()
    } else {
        false
    }
}

// check if file is socket
fn is_socket(path: &Path) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        metadata.file_type().is_socket()
    } else {
        false
    }
}