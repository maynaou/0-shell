use std::io::{self,Write};
use shell::*;
pub fn echo(args: &str) {   
    let mut temp = new_ligne(args,true);
    temp = handle_quoit(temp);
    let _ = writeln!(io::stdout(), "{}", temp);
}


