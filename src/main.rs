use std::collections::HashMap;

use std::io::{self, Write};

use crate::type_enum::Type;

mod type_enum;

fn main() {
    println!("Hello, world!");
    let mut vars: HashMap<String, Type> = HashMap::new();

    loop {
        let mut input = String::new();
        let mut args = Vec::new();

        print!(">>>");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(_) => continue,
        }

        input.trim().split(' ').for_each(|f| args.push(f));
        if args.is_empty() {
            continue;
        }
        match args[0] {
            "let" => {
                if args.len() <= 3 || args[2] != "=" {
                    println!("wrong usage of the let keyword");
                    continue;
                }

                match eval(&args[3..], &vars) {
                    Ok(res) => {
                        vars.insert(args[1].to_string(), res);
                    }
                    Err(error) => {
                        eprint!("{}", error);
                    }
                }

                continue;
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
            "help" => {
                println!("help page:");
                println!("Define a variable with the let keyword eg. let hallo = 2");
                println!("Evaluate a term, space separated eg. ( 1 + hallo ) * 2");
                println!("Quit the program with command q || quit");
                println!("Print out all variables with command vars");
                println!("Print this help page");
            }
            _ => {
                match eval(&args, &vars) {
                    Ok(res) => {
                        println!("res: {} = {}", res.get_type(), res);
                        vars.insert("res".to_string(), res);
                    }
                    Err(error) => {
                        eprint!("{}", error);
                    }
                }
                continue;
            }
        };
    }
}

fn eval(equation: &[&str], vars: &HashMap<String, Type>) -> Result<Type, String> {
    let length = equation.len();
    let mut value: Option<Type> = None;
    let mut operand = String::new();
    let mut is_first = true;
    let mut skip = 0;
    let mut old_idx = 0;
    for (mut idx, s) in equation.iter().enumerate() {
        if skip > 0 {
            skip -= 1;
            continue;
        }
        let mut s = String::from(*s);
        if vars.contains_key(&s) {
            s = format!("{}", vars.get(&s).unwrap().clone());
        }
        if s == "(" {
            match equation.iter().position(|s| s == &")") {
                Some(i) => match eval(&equation[idx + 1..i], vars) {
                    Ok(res) => {
                        skip = i - idx;
                        old_idx = idx;
                        idx = i;
                        if is_first {
                            value = Some(res);
                            is_first = false;
                            continue;
                        }
                        s = format!("{}", res);
                    }
                    Err(error) => return Err(error),
                },
                None => return Err("All parentheses have to be closed".to_string()),
            }
        } else if idx > 0 {
            old_idx = idx - 1;
        }
        if idx % 2 == 0 {
            if is_first {
                is_first = false;
                value = Some(Type::from_string(&s));
                continue;
            }

            match operand.as_str() {
                "+" => {
                    if idx + 1 != length && (equation[idx + 1] == "*" || equation[idx + 1] == "/") {
                        value = match value {
                            Some(v) => match eval(&equation[old_idx..], vars) {
                                Ok(res) => match v + res {
                                    Ok(rv) => return Ok(rv),
                                    Err(error) => return Err(error),
                                },
                                Err(error) => return Err(error),
                            },
                            None => None,
                        };
                        continue;
                    }
                    value = match value {
                        Some(v) => match v + Type::from_string(&s) {
                            Ok(rv) => Some(rv),
                            Err(error) => return Err(error),
                        },
                        None => None,
                    }
                }
                "-" => {
                    if idx + 1 != length && (equation[idx + 1] == "*" || equation[idx + 1] == "/") {
                        value = match value {
                            Some(v) => match eval(&equation[idx..], vars) {
                                Ok(res) => match v - res {
                                    Ok(rv) => return Ok(rv),
                                    Err(error) => return Err(error),
                                },
                                Err(error) => return Err(error),
                            },
                            None => None,
                        };
                        continue;
                    }
                    value = match value {
                        Some(v) => match v - Type::from_string(&s) {
                            Ok(rv) => Some(rv),
                            Err(error) => return Err(error),
                        },
                        None => None,
                    }
                }
                "*" => {
                    value = match value {
                        Some(v) => match v * Type::from_string(&s) {
                            Ok(rv) => Some(rv),
                            Err(error) => return Err(error),
                        },
                        None => None,
                    }
                }
                "/" => {
                    value = match value {
                        Some(v) => match v / Type::from_string(&s) {
                            Ok(rv) => Some(rv),
                            Err(error) => return Err(error),
                        },
                        None => None,
                    }
                }
                "%" => {
                    value = match value {
                        Some(v) => match v % Type::from_string(&s) {
                            Ok(rv) => Some(rv),
                            Err(error) => return Err(error),
                        },
                        None => None,
                    }
                }
                op => return Err(format!("Unknown operand {}", op)),
            }
            continue;
        }
        operand = s;
    }
    match value {
        Some(v) => Ok(v),
        None => Err(format!(
            "Unexpected error evaluating equation {}",
            equation.join("")
        )),
    }
}
