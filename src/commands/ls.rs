use std::fs;
use std::collections::HashMap;
use std::os::unix::fs::PermissionsExt; // for executable
use std::os::unix::fs::FileTypeExt;    // for pipe and socket
use std::path::Path;
use std::os::unix::fs::MetadataExt;
use std::time::{UNIX_EPOCH, SystemTime};
use users::{get_user_by_uid, get_group_by_gid};
use chrono::{DateTime, Local};
pub fn ls(flags : Vec<String>) -> (HashMap<String, Vec<String>>, bool) {
    let mut files: HashMap<String, Vec<String>> = HashMap::new();
    let validfomart = vformat(flags);
    let flagsvalid = validflags(&validfomart);
    if !flagsvalid {
        return (files,false)
    }
    let flaga = checka(&validfomart);
    let flagf = checkf(&validfomart);
    let flagl = checkl(&validfomart);
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
                    if let Some(vec) = files.get_mut(arg){
                        if flaga {
                            if flagl {
                                filename.insert_str(0, &generatel(filename.clone()))
                            }
                            if flagf  {
                                filename.push_str(symbol);
        
                            }
                            vec.push(filename);
                        }else{
                            if !filename.starts_with('.') {
                                if flagl {
                                    filename.insert_str(0, &generatel(filename.clone()))
                                }
                                if flagf  {
                                    filename.push_str(symbol);
            
                                }
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

fn generatel(file: String) -> String {
    let meta = match fs::symlink_metadata(&file) {
        Ok(m) => m,
        Err(_) => return "?????????".to_string(),
    };

    let nlink = meta.nlink();
    let uid = meta.uid();
    let gid = meta.gid();
    let user_name = get_user_by_uid(uid).map(|u| u.name().to_string_lossy().into_owned()).unwrap_or(uid.to_string());
    let group_name = get_group_by_gid(gid).map(|g| g.name().to_string_lossy().into_owned()).unwrap_or(gid.to_string());
    let size = meta.len();

    let modified: SystemTime = meta.modified().unwrap_or(UNIX_EPOCH);
    let datetime: DateTime<Local> = modified.into();
    let time_str = datetime.format("%b %e %H:%M").to_string();
    
    // let file_type = meta.file_type();
    // let symlink_target = if file_type.is_symlink() {
    //     match fs::read_link(&file) {
    //         Ok(target) => format!(" {} ->", target.display()),
    //         Err(_) => "".to_string(),
    //     }
    // } else {
    //     "".to_string()
    // };
    // println!("{}", symlink_target);
    
    let s = size.to_string().chars().count();
    let numbspaces =  5usize.saturating_sub(s);
    let space = " ".repeat(numbspaces);
    format!(
        "{} {} {} {} {}{} {} ",
        format_permissions(&meta),
        nlink,
        user_name,
        group_name,
        space,
        size,
        time_str
    )
}


fn format_permissions(meta: &fs::Metadata) -> String {
    let file_type = meta.file_type();
    let mode = meta.mode();

    let mut result = String::new();

    // first character: type
    if file_type.is_dir() {
        result.push('d');
    } else if file_type.is_symlink() {
        result.push('l');
    } else if file_type.is_fifo() {
        result.push('p');
    } else if file_type.is_socket() {
        result.push('s');
    } else if file_type.is_char_device() {
        result.push('c');
    } else if file_type.is_block_device() {
        result.push('b');
    } else {
        result.push('-'); // regular file
    }

    // owner, group, others permissions
    for i in (0..3).rev() { // user, group, others
        let shift = i * 3;
        result.push(if (mode >> (shift + 2)) & 1 != 0 { 'r' } else { '-' });
        result.push(if (mode >> (shift + 1)) & 1 != 0 { 'w' } else { '-' });
        result.push(if (mode >> shift) & 1 != 0 { 'x' } else { '-' });
    }

    result
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

fn checkl(flags : &Vec<String>) -> bool {
    for r in flags {
        if r.starts_with("-") {
            if  r.contains("l") {
                return true
            }
        }
    }
    false
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
fn checka(flags : &Vec<String>) -> bool {
    for r in flags {
        if r.starts_with("-") {
            if r.contains("a")  {
                return true
            }
        }
    }
    false
}
fn vformat(flags : Vec<String>) -> Vec<String> {
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
        metadata.permissions().mode() & 0o111 != 0
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