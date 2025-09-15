use chrono::{DateTime, Duration, Local};
use std::collections::HashMap;
use std::fs;
use std::os::unix::fs::FileTypeExt;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use users::{get_group_by_gid, get_user_by_uid};

// ---------------- Helper functions first ----------------

fn is_executable(path: &Path) -> bool {
    fs::metadata(path).map(|m| m.permissions().mode() & 0o111 != 0).unwrap_or(false)
}

fn is_pipe(path: &Path) -> bool {
    fs::metadata(path).map(|m| m.file_type().is_fifo()).unwrap_or(false)
}

fn is_socket(path: &Path) -> bool {
    fs::metadata(path).map(|m| m.file_type().is_socket()).unwrap_or(false)
}

// ---------------- Main ls function ----------------

pub fn ls(flags: Vec<String>) -> (HashMap<String, Vec<String>>, bool, bool) {
    let mut files: HashMap<String, Vec<String>> = HashMap::new();
    let validfomart = vformat(flags);
    if !validflags(&validfomart) {
        return (files, false, false);
    }

    let flaga = checka(&validfomart);
    let flagf = checkf(&validfomart);
    let flagl = checkl(&validfomart);
    let paths = getdirnames(&validfomart);
    let mut totalblocks = 0;
    // println!("{:?}", paths);
    for arg in &paths {
        files.insert(arg.clone(), Vec::new());
        let path = Path::new(arg);
        let is_symlink = match fs::symlink_metadata(path) {
            Ok(meta) => meta.file_type().is_symlink(),
            Err(_) => false,
        };
        //    println!("{:?}",arg);
        if path.is_dir() {
            // Directory logic
            match fs::read_dir(path) {
                Ok(iterdir) => {
                    let mut entries: Vec<PathBuf> = Vec::new();
                    if flaga {
                        entries.push(PathBuf::from(".")); 
                        entries.push(PathBuf::from(".."));
                    }
                    entries.extend(iterdir.filter_map(|res| res.ok().map(|e| e.path())));
                    entries.sort_by_key(|p| p.file_name().unwrap_or_default().to_os_string());

                    for entry in entries {
                        let full_path = if entry == Path::new(".") || entry == Path::new("..") {
                            path.join(&entry)
                        } else {
                            entry.clone()
                        };
                        // println!("{:?}",full_path);
       
                        let meta = match fs::symlink_metadata(&full_path) {
                            Ok(m) => m,
                            Err(_) => continue,
                        };
                        let file_type = meta.file_type();

                        let symbol = if file_type.is_dir() {
                            "/"
                        } else if file_type.is_symlink() {
                            "@"
                        } else if is_pipe(&full_path) {
                            "|"
                        } else if is_socket(&full_path) {
                            "="
                        } else if is_executable(&full_path) {
                            "*"
                        } else {
                            ""
                        };

                        let filename = if entry == Path::new(".") || entry == Path::new("..") {
                            entry.to_string_lossy().into_owned()
                        } else {
                            entry.file_name()
                                .map(|f| f.to_string_lossy().into_owned())
                                .unwrap_or_else(|| entry.to_string_lossy().into_owned())
                        };

                        if let Some(vec) = files.get_mut(arg) {
                            let mut display_name = filename.clone();

                            if flagl {
                                display_name.insert_str(0, &generatel(&full_path));

                                if file_type.is_symlink() {
                                    display_name.push_str(
                                        &match std::fs::read_link(&full_path) {
                                            Ok(target) => format!(" -> {}", target.display()),
                                            Err(_) => " -> ?".to_string(),
                                        },
                                    );
                                }
                            }

                            if flagf && (!flagl || !file_type.is_symlink()) {
                                display_name.push_str(symbol);
                            }

                            if flaga || !filename.starts_with('.') {
                                totalblocks += meta.blocks();
                                vec.push(display_name);
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
        } else if path.is_file() || is_symlink {
            // File logic
                if let Some(vec) = files.get_mut(arg) {
        let mut display_name = arg.clone();
        if flagl {
            display_name.insert_str(0, &generatel(path));

            // If it's a symlink, append its target
            if is_symlink {
                display_name.push_str(
                    &match std::fs::read_link(path) {
                        Ok(target) => format!(" -> {}", target.display()),
                        Err(_) => " -> ?".to_string(),
                    },
                );
            }
        }

        vec.push(display_name);
    }
        } else {
            // Path does not exist
                println!("...{:?}",path);
                if let Some(vec) = files.get_mut(arg) {
                    vec.push(format!("ls: cannot access '{}': No such file or directory", arg));
            }
        }
        let khdam = totalblocks / 2;
        if flagl {
            if let Some(vec) = files.get_mut(arg) {
               // format!("total {}",khdam)
               vec.insert(0, format!("total {}",khdam));
            }
        }
         totalblocks = 0;
    }


    (files, true, flagl)
}


// ---------------- Supporting functions ----------------

fn generatel(file: impl AsRef<Path>) -> String {
    let meta = match fs::symlink_metadata(&file) {
        Ok(m) => m,
        Err(_) => return "?????????----".to_string(),
    };

    let nlink = meta.nlink();
    let uid = meta.uid();
    let gid = meta.gid();
    let user_name = get_user_by_uid(uid)
        .map(|u| u.name().to_string_lossy().into_owned())
        .unwrap_or(uid.to_string());
    let group_name = get_group_by_gid(gid)
        .map(|g| g.name().to_string_lossy().into_owned())
        .unwrap_or(gid.to_string());
    let size = meta.len();

    let modified: SystemTime = meta.modified().unwrap_or(UNIX_EPOCH);
    let datetime: DateTime<Local> = modified.into();
    let datetime_plus_one = datetime + Duration::hours(1);
    let time_str = datetime_plus_one.format("%b %e %H:%M").to_string();

    let s = size.to_string().chars().count();
    let space = " ".repeat(5usize.saturating_sub(s));

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

    result.push(if file_type.is_dir() {
        'd'
    } else if file_type.is_symlink() {
        'l'
    } else if file_type.is_fifo() {
        'p'
    } else if file_type.is_socket() {
        's'
    } else if file_type.is_char_device() {
        'c'
    } else if file_type.is_block_device() {
        'b'
    } else {
        '-'
    });

    for i in (0..3).rev() {
        let shift = i * 3;
        result.push(if (mode >> (shift + 2)) & 1 != 0 { 'r' } else { '-' });
        result.push(if (mode >> (shift + 1)) & 1 != 0 { 'w' } else { '-' });
        let exec = (mode >> shift) & 1 != 0;
        result.push(match shift {
            6 => if (mode & 0o4000) != 0 { if exec { 's' } else { 'S' } } else { if exec { 'x' } else { '-' } },
            3 => if (mode & 0o2000) != 0 { if exec { 's' } else { 'S' } } else { if exec { 'x' } else { '-' } },
            0 => if (mode & 0o1000) != 0 { if exec { 't' } else { 'T' } } else { if exec { 'x' } else { '-' } },
            _ => if exec { 'x' } else { '-' }
        });
    }

    result
}

fn getdirnames(flags: &Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut found = false;
    for arg in flags {
        if !arg.starts_with("-") && !arg.is_empty() {
            found = true;
            result.push(arg.clone());
        }
    }
    if !found {
        result.push(".".to_string());
    }
    result
}

fn validflags(flags: &Vec<String>) -> bool {
    let valid_flags = ['a', 'F', 'l'];
    for r in flags {
        if r.starts_with("-") {
            for ch in r.chars().skip(1) {
                if !valid_flags.contains(&ch) {
                    return false;
                }
            }
        }
    }
    true
}

fn checkl(flags: &Vec<String>) -> bool {
    flags.iter().any(|r| r.starts_with('-') && r.contains('l'))
}

fn checkf(flags: &Vec<String>) -> bool {
    flags.iter().any(|r| r.starts_with('-') && r.contains('F'))
}

fn checka(flags: &Vec<String>) -> bool {
    flags.iter().any(|r| r.starts_with('-') && r.contains('a'))
}

fn vformat(flags: Vec<String>) -> Vec<String> {
    if !flags.is_empty() && !flags[0].is_empty() {
        return flags[0]
            .split(' ')
            .filter_map(|s| {
                let s = s.trim();
                if s.is_empty() { None } else { Some(s.to_string()) }
            })
            .collect();
    }
    flags
}
