use std::fs;
use std::collections::HashMap;
use std::os::unix::fs::PermissionsExt; // for executable
use std::os::unix::fs::FileTypeExt;    // for pipe and socket
use std::path::Path;
use std::os::unix::fs::MetadataExt;
use std::time::{UNIX_EPOCH, SystemTime};
use users::{get_user_by_uid, get_group_by_gid};
use chrono::{DateTime, Local, Duration};
pub fn ls(flags : Vec<String>) -> (HashMap<String, Vec<String>>, bool, bool, u64) {
    let mut files: HashMap<String, Vec<String>> = HashMap::new();
    let validfomart = vformat(flags);
    let flagsvalid = validflags(&validfomart);
    if !flagsvalid {
        return (files,false,false,0)
    }
    let flaga = checka(&validfomart);
    let flagf = checkf(&validfomart);
    let flagl = checkl(&validfomart);
    let  path = getdirnames(&validfomart);
    let mut totalbolcks = 0;
    for arg in &path {
        files.insert(arg.to_string(), Vec::new());
        match fs::read_dir(&arg) {
            Ok(iterdir) => {
                let mut entries: Vec<fs::DirEntry> = iterdir.filter_map(Result::ok).collect();
                if flaga {
                    let current_dir = Path::new(arg);
                    let dot = current_dir.join(".");
                    let dotdot = current_dir.join("..");
                    // println!("{:?}", fs::metadata(dot).file_name());
                    
                            
                    }
                entries.sort_by_key(|e| e.file_name());
                //    println!("{:?}",entries);
                for iter in entries {
                    // println!("{:?}", iter);
                    let typofiter = iter.file_type().unwrap();
                    let symbol = match () {
                        _ if typofiter.is_dir() => "/",
                        _ if typofiter.is_symlink() => "@",
                        _ if is_pipe(&iter.path()) => "|",
                        _ if is_socket(&iter.path()) => "=",
                        _ if is_executable(&iter.path()) => "*",
                        _ => "",
                    };
                    // let aloo = fs::metadata(iter.path()).unwrap();
                    // let blocks = aloo.blocks();
                    // totalbolcks += blocks;

                    let filename_os = iter.file_name();
                    let mut filename = filename_os.to_string_lossy().into_owned();
                    // let metadata = fs::metadata(path).unwrap();
                    if let Some(vec) = files.get_mut(arg) {
                        if flaga {
                            if flagl {

                                    let aloo = fs::metadata(iter.path()).unwrap();
                                    let blocks = aloo.blocks();
                                    totalbolcks += blocks;
                                  filename.insert_str(0, &generatel(iter.path()));
                                if typofiter.is_symlink() {
                                    match std::fs::read_link(&iter.path()) {
                                        Ok(target) => filename.push_str(&format!(" -> {}", target.display())),
                                        Err(_) => filename.push_str(" -> ?"),
                                    }
                                }
                            }
                            if flagf {
                                if flagl && !typofiter.is_symlink() {
                                    filename.push_str(symbol);
                                } else if !flagl {
                                    filename.push_str(symbol);
                                }
                            }
                            vec.push(filename);
                        } else {
                            if !filename.starts_with('.') {
                                if flagl {
                                        let aloo = fs::metadata(iter.path()).unwrap();
                                        let blocks = aloo.blocks();
                                        totalbolcks += blocks;
                                    filename.insert_str(0, &generatel(iter.path()));
                                    if typofiter.is_symlink() {
                                        match std::fs::read_link(&iter.path()) {
                                            Ok(target) => filename.push_str(&format!(" -> {}", target.display())),
                                            Err(_) => filename.push_str(" -> ?"),
                                        }
                                    }
                                }
                                if flagf {
                                    if flagl && !typofiter.is_symlink() {
                                        filename.push_str(symbol);
                                    } else if !flagl {
                                        filename.push_str(symbol);
                                    }
                                }
                                vec.push(filename);
                            }
                        }
                    }
                }
            }
            Err(_) => {
                if let Some(vec) = files.get_mut(arg) {
                    vec.push(format!("ls: cannot access '{}': No such file or directory", arg));
                }
            }
        }
        
    }

    let khdam = totalbolcks/2;
    
    (files, true, flagl, khdam)
}

fn generatel(file: impl AsRef<Path>) -> String {
    // println!("{:?}",file);
    let meta = match fs::symlink_metadata(&file) {
        Ok(m) => m,
        Err(_) => return "?????????----".to_string(),
    };

    let nlink = meta.nlink();
    let uid = meta.uid();
    let gid = meta.gid();
    let user_name = get_user_by_uid(uid).map(|u| u.name().to_string_lossy().into_owned()).unwrap_or(uid.to_string());
    let group_name = get_group_by_gid(gid).map(|g| g.name().to_string_lossy().into_owned()).unwrap_or(gid.to_string());
    let size = meta.len();

    let modified: SystemTime = meta.modified().unwrap_or(UNIX_EPOCH);
    let datetime: DateTime<Local> = modified.into();
    let datetime_plus_one = datetime + Duration::hours(1);
    let time_str = datetime_plus_one.format("%b %e %H:%M").to_string();
    
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
    // println!("{}",mode);
    let mut result = String::new();
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
        result.push('-');
    }

    for i in (0..3).rev() {
        let shift = i * 3;
        result.push(if (mode >> (shift + 2)) & 1 != 0 { 'r' } else { '-' });
        result.push(if (mode >> (shift + 1)) & 1 != 0 { 'w' } else { '-' });
        let exec = (mode >> shift) & 1 != 0;
        if shift == 6 {
            if (mode & 0o4000) != 0 {
                result.push(if exec { 's' } else { 'S' });
            } else {
                result.push(if exec { 'x' } else { '-' });
            }
        } else if shift == 3 {

            if (mode & 0o2000) != 0 {
                result.push(if exec { 's' } else { 'S' });
            } else {
                result.push(if exec { 'x' } else { '-' });
            }
        } else if shift == 0 {

            if (mode & 0o1000) != 0 {
                result.push(if exec { 't' } else { 'T' });
            } else {
                result.push(if exec { 'x' } else { '-' });
            }
        }
    }

    result
}

fn getdirnames(flags : &Vec<String>) -> Vec<String> {
    let mut result : Vec<String> = Vec::new();
    let mut bol = false;
    // println!("{:?}", flags);
    for arg in flags {
        if !arg.starts_with("-") && arg != "" {
            bol = true;
            result.push(arg.clone());
        }
    }
    if  !bol {
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