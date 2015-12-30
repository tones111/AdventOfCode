use std::io;

fn main() {
    let mut code_len = 0usize;
    let mut mem_len = 0usize;
    let mut recode_len = 0usize;
    let mut recode = String::new();

    let mut line = String::new();
    loop {
        line.clear();
        io::stdin().read_line(&mut line).unwrap();
        let line = line.trim_right();

        if line.len() == 0 {
            break;
        }

        let mut mem = 0u8;
        let mut escape = false;
        let mut escape_lit = 0u8;

        recode.clear();
        recode.push('"');
        for c in line.chars() {
            if escape {
                if escape_lit > 0 {
                    escape_lit -= 1;
                    if escape_lit == 0 {
                        escape = false;
                        mem += 1;
                    }
                    recode.push(c);
                    continue;
                }

                match c {
                    '\\' | '"' => {
                        mem += 1;
                        recode.push('\\');
                        recode.push(c);
                        escape = false;
                    }
                    'x' => {
                        escape_lit = 2;
                        recode.push(c);
                    }
                    _ => {
                        println!("invalid escape ({})", c);
                    }
                }
            } else if c == '\\' {
                recode.push_str(r"\\");
                escape = true;
            } else if c == '"' {
                recode.push_str(r#"\""#);
            } else {
                recode.push(c);
                mem += 1;
            }
        }
        recode.push('"');

        code_len += line.len();
        mem_len += mem as usize;
        recode_len += recode.len();
    }
    println!("{} - {} = {}", code_len, mem_len, code_len - mem_len);
    println!("{} - {} = {}", recode_len, code_len, recode_len - code_len);
}
