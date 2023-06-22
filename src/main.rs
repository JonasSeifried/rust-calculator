use std::collections::HashMap;

use std::io::{self, Write};

use crate::type_enum::Type;

mod type_enum;

fn main() {
    println!("run help for instructions");
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

                match eval(equation, &vars) {
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

            _ => match eval(&args.join(""), &vars) {
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
fn eval(equation: &str, vars: &HashMap<String, Type>) -> Result<Type, String> {
    let mut word = String::new();
    let mut operand: char = '\0';
    let mut res: Result<Type, String> = Err("Error: Empty equation".to_string());
    let mut open_parentheses = 0;
    let mut idx_of_parentheses: usize = 0;
    let mut word_start_idx = 0;
    let mut is_first = true;
    let mut op_expected = false;
    for (idx, c) in equation.chars().enumerate() {
        if open_parentheses != 0 {
            match c {
                '(' => open_parentheses += 1,
                ')' => {
                    open_parentheses -= 1;
                    if open_parentheses == 0 {
                        op_expected = true;
                        if is_first {
                            if idx - idx_of_parentheses == 1 {
                                continue;
                            }
                            res = eval(&equation[idx_of_parentheses + 1..idx], vars);
                            is_first = false;

                            continue;
                        }
                        if idx - idx_of_parentheses == 1 {
                            continue;
                        }
                        let result = eval(&equation[idx_of_parentheses + 1..idx], vars)?;

                        res = match operand {
                            '+' => res? + result,
                            '-' => res? - result,
                            '*' => res? * result,
                            '/' => res? / result,
                            '%' => res? % result,
                            _ => {
                                return Err(format!(
                                    "Unexpected Error: {} is not an valid operator",
                                    operand
                                ))
                            }
                        };
                        continue;
                    }
                }
                _ => continue,
            }
            continue;
        }

        match c {
            '+' | '-' | '*' | '/' | '%' => {
                if !op_expected {
                    return Err(format!("{} can't be followed by {}", operand, c));
                }
                let order_of_op = c != '+' && c != '-';
                if !word.is_empty() {
                    let result = var_or_string(&word, vars);
                    res = match res {
                        Ok(_) => match operand {
                            '+' => {
                                if order_of_op {
                                    return res? + eval(&equation[word_start_idx..], vars)?;
                                }
                                res? + result
                            }
                            '-' => {
                                if order_of_op {
                                    return res? - eval(&equation[word_start_idx..], vars)?;
                                }
                                res? - result
                            }
                            '*' => res? * result,
                            '/' => res? / result,
                            '%' => res? % result,
                            _ => {
                                return Err(format!(
                                    "Unexpected Error: {} is not an valid operator",
                                    operand
                                ))
                            }
                        },
                        Err(_) => Ok(result),
                    };
                    word.clear();
                }
                op_expected = false;
                is_first = false;
                operand = c;
                continue;
            }
            '(' => {
                open_parentheses += 1;
                idx_of_parentheses = idx;
            }
            ')' => return Err("parentheses must be opened before being closed".to_string()),
            ' ' => (),
            _ => {
                if word.is_empty() {
                    word_start_idx = idx
                }
                word.push(c);
                op_expected = true;
            }
        }
    }
    if !word.is_empty() {
        let result = var_or_string(&word, vars);
        res = match res {
            Ok(res) => match operand {
                '+' => res + result,
                '-' => res - result,
                '*' => res * result,
                '/' => res / result,
                '%' => res % result,
                _ => {
                    return Err(format!(
                        "Unexpected Error: {} is not an valid operator",
                        operand
                    ))
                }
            },
            Err(_) => Ok(result),
        }
    }

    if open_parentheses != 0 {
        return Err("All parentheses must be closed!".to_string());
    }
    res
}

fn var_or_string(s: &str, vars: &HashMap<String, Type>) -> Type {
    vars.get(s).unwrap_or(&Type::from_string(s)).clone()
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
