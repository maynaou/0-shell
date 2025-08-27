use std::io::{self,Write};

pub fn echo(args: &str) {
    let _ = writeln!(io::stdout(), "{}", args);
}

























    // let mut env_var: HashMap<String,String> = HashMap::new(); 
    // loop {
        // let mut s = String::new();

        // print!("$ ");
        // if io::stdout().flush().is_err() {
        //      return;
        // }

        // if io::stdin().read_line(&mut s).is_err() {
        //     eprintln!("Erreur lecture stdin");
        //     break;
        // }

        // let var = s.trim_end();
        // println!("--{}",var);

        // if let Some((key,value)) = s.split_once("=") {
        //     env_var.insert(key.to_string(), value.trim_end().to_string());
        //     println!("{:?}",env_var);
        //     continue;
        // }
        // let conc : String = command[1..].concat();
        // // let mut command = vec![];
        // let mut s = String::new() ;
        // for c in conc.chars()  { 
        //     if c == '\"' {
        //         // command.push(s.clone());
        //         continue;
        //     }else {
        //         s.push(c);
        //     }
        // }

        // command[1] = &s;
        // println!("--command {:?}", command);
        //   for arg in command.iter_mut() {
        //     // println!("{}",arg);
        //     if arg.starts_with("$") {
        //         let value = match arg.ends_with("\""){
        //                true => &arg[1..arg.len()-1],
        //                false  => &arg[1..],
        //         }; 
        //         if let Some(val) = env_var.get(value) {
        //             *arg = val;
        //         }else {
        //             *arg = "";
        //         }
        //     }
        // }
        //  println!("{:?}",command);