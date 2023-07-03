use std::io::{self, Write};

fn main() {
    println!("run help for instructions");
    let mut vars = calculator::var_init();

    loop {
        let mut input = String::new();
        let mut args = Vec::new();

        print!(">>>");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(_) => continue,
        }
        if !input.is_ascii() {
            eprintln!("UTF-8 is unsupported for now");
        }
        input = input
            .chars()
            .filter(|c| !c.is_whitespace() || c == &' ')
            .collect();
        input.split(' ').for_each(|f| args.push(f));
        if args.is_empty() {
            continue;
        }
        match args[0] {
            "let" => {
                let s = args[1..].join("");
                if !s.contains('=') {
                    eprintln!("command is missing '='");
                    continue;
                }
                let parts: Vec<&str> = s.split('=').collect();

                if parts.len() != 2 {
                    eprintln!("wrong usage of the let keyword");
                    continue;
                }
                let var_name = parts[0];
                let equation = parts[1];
                if let Err(err) = is_valid_var(var_name) {
                    eprintln!("{}", err);
                    continue;
                }

                match calculator::eval(equation, Some(&vars)) {
                    Ok(res) => {
                        vars.insert(var_name.to_string(), res);
                    }
                    Err(error) => {
                        eprintln!("{}", error);
                    }
                }
            }
            "q" | "quit" => {
                break;
            }
            "vars" => {
                println!("Variables:");
                for (k, v) in &vars {
                    println!("{}: {} => {}", k, v.get_type(), v,)
                }
            }
            "help" => println!("{}", help_string()),

            _ => match calculator::eval(&args.join(""), Some(&vars)) {
                Ok(res) => {
                    println!("res: {} = {}", res.get_type(), res);
                    vars.insert("res".to_string(), res);
                }
                Err(error) => {
                    eprintln!("{}", error);
                }
            },
        };
    }
}

fn is_valid_var(s: &str) -> Result<(), String> {
    if s.chars().next().unwrap().is_numeric() {
        return Err("Variables cannot start with number".to_string());
    }
    for c in s.chars() {
        if !c.is_alphanumeric() && c != '_' {
            return Err(format!(
                "Variables must be alphanumeric! '{}' is not allowed",
                c,
            ));
        }
    }
    Ok(())
}

fn help_string() -> String {
    "help page:
    Define a variable with the let keyword eg. let hallo = 2
    Evaluate a term, space separated eg. ( 1 + hallo ) * 2
    Quit the program with command q || quit
    Print out all variables with command vars
    Print this help page"
        .to_string()
}
