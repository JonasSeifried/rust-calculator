use std::collections::HashMap;
use type_enum::Type;

pub mod type_enum;

pub fn var_init() -> HashMap<String, Type> {
    HashMap::new()
}

pub fn eval(equation: &str, vars: Option<&HashMap<String, Type>>) -> Result<Type, String> {
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

fn var_or_string(s: &str, vars: Option<&HashMap<String, Type>>) -> Type {
    match vars {
        Some(vars) => vars.get(s).unwrap_or(&Type::from(s)).clone(),
        None => Type::from(s),
    }
}
