use std::fs;
use std::io;
use std::path::Path;

pub fn mv(args: &str) {
    let parts: Vec<&str> = args.split_whitespace().collect();
    
    if parts.len() < 2 {
        eprintln!("mv: missing file operand");
        return;
    }
    
    let destination = parts[parts.len() - 1]; 
    let sources = &parts[0..parts.len() - 1];
    
    let dest_is_dir = Path::new(destination).is_dir();
    let dest_exists = Path::new(destination).exists();
    
    if sources.len() > 1 && !dest_is_dir && dest_exists {
        eprintln!("mv: target '{}' is not a directory", destination);
        return;
    }
    
    if sources.len() == 1 && Path::new(sources[0]).is_dir() && dest_exists && !dest_is_dir {
        eprintln!("mv: cannot overwrite non-directory '{}' with directory '{}'", 
                 destination, sources[0]);
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
                    eprintln!("mv: cannot stat '{}': No such file or directory", source);
                } else {
                    eprintln!("mv: cannot move '{}' to '{}': {}", source, final_dest, e);
                }
            }
        }
    }
}