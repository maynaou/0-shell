use std::io::{self,Write};

pub fn echo(args: &str) {   
    let temp = new_ligne(args,true);

    let _ = writeln!(io::stdout(), "{}", temp.trim_matches(|c| c == '"' || c == '\''));
}


pub fn new_ligne(s : &str,a : bool) -> String {
        let mut temp = s.to_string();
        let mut count = 0;
        let mut j = 0;
        let mut b = false;
        if s.starts_with('\"') && s.ends_with('\"') || s.starts_with('\'') && s.ends_with('\'') {
                b = true;
        }
        for c in s.chars(){
           if c == '\\' {
               count += 1;
           }else if count > 0 {
                if count == 1 && !b {
                        temp = s.replace(&s[j..j+count+1], &s[j+count..j+count+1]);
                } else if ((count == 2 || count == 3 && !b ) && a) || count == 1 && b {
                    println!("-----------");
                   let res =  match &s[j+count..j+count+1] {
                          "n" =>"\n",
                          "r" => "\r",
                          "t" => "\t",
                          "v" => "\x0B", 
                          "f" => "\x0C",
                          "a" => "\x07",
                          "\\n" => "\n",
                          _ => &s[j+count-1..j+count+1]
                   };
                     temp = s.replace(&s[j..j+count+1], res);
                 }else if !a {
                     let v =  count / 2;
                     temp = s.replace(&s[j..j+count+1], &s[j+count-v..j+count+1]);
                 }else{
                     temp = s.replace(&s[j..j+count+1], &s[j+count-1..j+count]);
                 }
                 j += count + 1;
                 count = 0;
             }else {
                j += 1;
             }
         }

         temp
}