<<<<<<< HEAD
<<<<<<< HEAD
use super::cat;
use std::fs;
use std::path::Path;

pub fn cp(args: &str) {
    let parts: Vec<&str> = args.split_whitespace().collect();
    
    if parts.len() < 2 {
        eprintln!("cp: missing file operand");
        return;
    }
    
    let destination = parts[parts.len() - 1]; //[a,b,c]  ---> c | [a,b] --> b
    let sources = &parts[0..parts.len() - 1]; // [a,b]   ---> [a]
    
    let dest_is_dir = Path::new(destination).is_dir(); // true : directory false file or tkhanticha
    
    if sources.len() > 1 && !dest_is_dir {
        eprintln!("cp: target '{}' is not a directory", destination);
        return;
    }
    
    for source in sources {
        let final_dest = if dest_is_dir {
            format!("{}/{}", destination, source)
        } else {
            destination.to_string()
        };
        
        match fs::copy(source, &final_dest) {
            Ok(_) => (),
            Err(e) => eprintln!("cp: cannot copy '{}' to '{}': {}", source, final_dest, e)
        }
    }
}

