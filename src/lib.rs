use std::collections::HashMap;
use type_enum::Type;

pub mod type_enum;

pub fn var_init() -> HashMap<String, Type> {
    HashMap::new()
}

pub fn eval(equation: &str, vars: Option<&HashMap<String, Type>>) -> Result<Type, String> {
    let mut word = String::new();
    let mut right: Option<Type> = None;
    let mut operand: char = '\0';
    let mut res: Result<Type, String> = Err("Error: Empty equation".to_string());
    let mut open_parentheses = 0;
    let mut idx_of_parentheses: usize = 0;
    let mut is_first = true;
    let mut op_expected = false;
    for (idx, c) in equation.chars().enumerate() {
        if open_parentheses != 0 {
            match c {
                '(' => open_parentheses += 1,
                ')' => {
                    open_parentheses -= 1;
                    if open_parentheses == 0 {
                        if idx - idx_of_parentheses == 1 {
                            continue;
                        }
                        op_expected = true;
                        /*let r = eval(&equation[idx_of_parentheses + 1..idx], vars)?;
                        res = handle_op(res, r, operand, is_first);
                        is_first = false;
                        */
                        right = Some(eval(&equation[idx_of_parentheses + 1..idx], vars)?);

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
                    if c == '-' {
                        word.push(c);
                        continue;
                    }
                    return Err(format!("{} can't be followed by {}", operand, c));
                }

                if !word.is_empty() {
                    right = Some(var_or_string(&word, vars));
                    word.clear();
                }

                //correct order of operation e.g. * before +
                let order_of_op = c != '+' && c != '-' && (operand == '+' || operand == '-');
                if order_of_op {
                    return handle_op(
                        res,
                        eval(
                            &format!("{}{}{}", right.unwrap(), c, equation[idx + 1..].to_owned()),
                            vars,
                        )?,
                        operand,
                        is_first,
                    );
                }
                op_expected = false;

                if let Some(r) = right.clone() {
                    res = handle_op(res, r, operand, is_first);
                    is_first = false;
                }
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
                word.push(c);
                op_expected = true;
            }
        }
    }
    if !word.is_empty() {
        if word == "-" {
            word = "-1".to_string();
            operand = '*';
        }
        right = Some(var_or_string(&word, vars));
    }
    if let Some(r) = right {
        res = handle_op(res, r, operand, is_first);
    }

    if open_parentheses != 0 {
        return Err("All parentheses must be closed!".to_string());
    }
    res
}

fn handle_op(l: Result<Type, String>, r: Type, op: char, is_first: bool) -> Result<Type, String> {
    if is_first {
        return Ok(r);
    }
    match op {
        '+' => l? + r,
        '-' => l? - r,
        '*' => l? * r,
        '/' => l? / r,
        '%' => l? % r,
        _ => Err(format!("Unexpected Error: {} is not an valid operator", op)),
    }
}

fn var_or_string(s: &str, vars: Option<&HashMap<String, Type>>) -> Type {
    match vars {
        Some(vars) => vars.get(s).unwrap_or(&Type::from(s)).clone(),
        None => Type::from(s),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eval_assert(eq: &str, should_be: &str) -> Result<(), String> {
        let res = eval(eq, None);
        assert_eq!(
            res.clone()?,
            Type::from(should_be),
            "\n Equation: {} != {} (instead: {})\n",
            eq,
            should_be,
            res?
        );
        Ok(())
    }

    #[test]
    fn arithmetic_0() -> Result<(), String> {
        eval_assert("5 * 1.5 / (44.0 / (2*2*2*2))", "2.727272727272727272727")?;
        Ok(())
    }

    #[test]
    fn arithmetic_1() -> Result<(), String> {
        eval_assert("555 * 22 + ()212 + 21 * 6 / 2.0 + 5", "12490.0")?;
        Ok(())
    }

    #[test]
    fn arithmetic_2() -> Result<(), String> {
        eval_assert("5.0 / 5 / 5 * 5", "1.0")?;
        Ok(())
    }
    #[test]
    fn arithmetic_3() -> Result<(), String> {
        eval_assert("hi * 5", "hihihihihi")?;
        Ok(())
    }
    #[test]
    fn arithmetic_4() -> Result<(), String> {
        eval_assert("(2 * (777 / 12))", "128")?;
        Ok(())
    }
    #[test]
    fn arithmetic_5() -> Result<(), String> {
        eval_assert("(((12 * 3) - 24) / 6.0) + ((18 % 5) * 2)", "8.0")?;
        Ok(())
    }
    #[test]
    fn arithmetic_6() -> Result<(), String> {
        eval_assert("(15 + 4.0) / (3 - 1) * (10 % 7)", "28.5")?;
        Ok(())
    }
    #[test]
    fn arithmetic_7() -> Result<(), String> {
        eval_assert("((7 + 2) * 3 - (4 / 2.0)) + (5 % 3)", "27.0")?;
        Ok(())
    }
    #[test]
    fn arithmetic_8() -> Result<(), String> {
        eval_assert("-2 + (-3) * 0.5", "-3.5")?;
        Ok(())
    }
    #[test]
    fn arithmetic_9() -> Result<(), String> {
        eval_assert("(-0.5) * (0.2 - 0.4) + (-0.1)", "0.0")?;
        Ok(())
    }
    #[test]
    fn arithmetic_10() -> Result<(), String> {
        eval_assert("(-1) / 4.0 + (-0.25) * 2", "-0.75")?;
        Ok(())
    }
    #[test]
    fn arithmetic_11() -> Result<(), String> {
        eval_assert("(2 * (9 - 5) + 12) % 7 / 3.0", "2.0")?;
        Ok(())
    }
    #[test]
    fn arithmetic_12() -> Result<(), String> {
        eval_assert("((8 - 3) / (2.0 * 4)) - (7 + 1) * 5", "-39.375")?;
        Ok(())
    }
    #[test]
    fn arithmetic_13() -> Result<(), String> {
        eval_assert("((10 - 3) / 2.0 + 9) - (5 * 2 - 7)", "9.5")?;
        Ok(())
    }
    #[test]
    fn arithmetic_14() -> Result<(), String> {
        eval_assert("(6 * 7 - 12) / ((4 + 2) / 3.0)", "15.0")?;
        Ok(())
    }
    #[test]
    fn arithmetic_15() -> Result<(), String> {
        eval_assert(
            "((2 * (9 - 5) + 12) / 7.0) * ((3 + 6) - (8 * 2)) + ((5 - 1) / (2.0 + 3))",
            "-19.2",
        )?;
        Ok(())
    }
}
