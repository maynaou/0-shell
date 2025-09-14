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

pub fn handle_quoit(mut s : String) -> String {
    if s.contains('"') {
        s = s.trim_matches('\"').to_string();
        while let Some(pos) = s.find('"')  {
             s.remove(pos);
        } 

    } else if s.contains('\'') {
        s = s.trim_matches('\'').to_string();
        while let Some(pos) = s.find('\'') {
             s.remove(pos);
        }
    }
    s
}
#[derive(Debug)]
pub struct Format {
    pub count: usize,
    pub s: String,
}

pub fn format_handle(vec: Vec<&str>,flag : &str) -> Format {
    let mut count = 0;
    let mut s = String::new();
    for i in 0..vec.len() {
        for c in vec[i].chars() {
            if c == '-' {
                count += 1;
            } else {
                break;
            }
        }

        if count == 0 {
            s =  vec[i].to_string();
        }

        match flag {
            "cat" | "mkdir" | "cp" | "mv" if count > 0 && (count < 2 || !vec[i][count..].is_empty() || count > 2)  => {
                return Format {
                    count,
                    s: vec[i].to_string(),
                };
            },

            "rm"  if count > 0 && (count < 2 || !vec[i][count..].is_empty() || count > 3)  => {
                if vec[i] != "-r" {
                      return Format {
                      count,
                      s: vec[i].to_string(),
                     };
                }
                count = 0;
            }

             "pwd" | "cd" if  !vec[i][count..].is_empty() || count > 0 => {
                      return Format {
                      count,
                      s: vec[i].to_string(),
                     };
              }

            _ => {
                 count = 0;
                 continue
            },
         }
         
    }

    return Format {
        count: 0,
        s: s,
    };
}