#![warn(missing_docs)]

//! Library for evaluating mathematical equations.
//!
//! This library was created for learning purposes and provides functionality for evaluating mathematical equations.
//! It supports basic arithmetic operations and variable substitution.
//!
//! # Example
//!
//! ```
//! use std::collections::HashMap;
//! use calculator::{eval, vars_init, type_enum::Type};
//!
//! let mut variables = vars_init();
//!
//! // Inserting variables into the HashMap
//! variables.insert("x".to_string(), Type::Int(42));
//! variables.insert("y".to_string(), Type::Float(3.14));
//!
//! // Using the `eval` function to evaluate an equation
//! let equation = "2 * x + y";
//! let result = eval(equation, Some(&variables));
//!
//! assert_eq!(result, Type::from("87.14"));
//! }
//! ```
//!
//! # Notes
//!
//! This library supports the following arithmetic operations: addition (`+`), subtraction (`-`), multiplication (`*`), division (`/`), and modulo (`%`).
//! It also supports parentheses to control the order of operations.
//! The `eval` function can be used to evaluate equations containing variables. Variable names should be stored in a `HashMap` with their corresponding values as instances of the `Type` enum.
//!
//! Make sure to check the documentation of the `eval` function and the `Type` enum for more details.
//!
//! ## Features
//!
//! * Arithmetic operations: `+`, `-`, `*`, `/`, `%`
//! * Parentheses for grouping subexpressions
//! * Variable substitution
//!
//! ## Limitations
//!
//! * Supports integer (`i32`), float (`f64`), and string (`String`) types only
//! * Limited mathematical functionality
//!
//! # Getting Started
//!
//! To use the calculator library, add the following dependency to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! calculator = "0.1.0"
//! ```
//!
//! # Examples
//!
//! ## Evaluating an equation without variables
//!
//! ```rust
//! use calculator::eval;
//!
//! let equation = "2 * (3 + 4)";
//! let result = eval(equation, None);
//!
//! assert_eq!(result, Type::from("14"));
//! ```
//!
//! ## Evaluating an equation with variables
//!
//! ```rust
//! use std::collections::HashMap;
//! use calculator::{eval, vars_init, type_enum::Type};
//!
//! let mut variables = vars_init();
//!
//! // Inserting variables into the HashMap
//! variables.insert("x".to_string(), Type::Int(42));
//! variables.insert("y".to_string(), Type::Float(3.14));
//!
//! // Using the HashMap to evaluate an equation with variables
//! let equation = "2 * x + y";
//! let result = eval(equation, Some(&variables));
//!
//! assert_eq!(result, Type::from("87.14"));
//! ```
//!
//! # Contributing
//!
//! Contributions to the calculator library are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request on the [GitHub repository](https://github.com/your_username/calculator).
//!
//! Please ensure that your code follows the established coding style and conventions, and includes appropriate tests and documentation.
//!
//! # License
//!
//! This library is licensed under the MIT License.

use std::collections::HashMap;
use type_enum::Type;

pub mod type_enum;

/// Initializes an empty `HashMap` to store variables and their corresponding values as instances of ´Type´.
///
/// # Returns
///
/// A newly created `HashMap` that can be used to store variables and their values (`Type`).
///
/// # Example
///
/// ```
/// use std::collections::HashMap;
/// use calculator::{vars_init, eval, type_enum::Type};
///
/// let mut variables = vars_init();
///
/// // Inserting variables into the HashMap
/// variables.insert("x".to_string(), Type::Int(42));
/// variables.insert("y".to_string(), Type::Float(3.14));
///
/// // Using the HashMap to evaluate an equation with variables
/// let equation = "2 * x + y";
/// let result = eval(equation, Some(&variables));
///
/// match result {
///     Ok(value) => println!("Result: {}", value),
///     Err(error) => println!("Error: {}", error),
/// }
/// ```
///
/// # Notes
///
/// This function creates an empty `HashMap` that can be used to store variables and their corresponding values.
/// The `HashMap` should have variable names as keys (`String`) and their corresponding values (`Type`).
/// The `Type` enum can represent different types such as integers, floats, or strings.
/// The resulting `HashMap` can be used as input to the `eval` function for evaluating equations containing variables.

pub fn vars_init() -> HashMap<String, Type> {
    HashMap::new()
}
/// Evaluates a mathematical equation represented as a string and returns the result.
///
/// # Arguments
///
/// * `equation` - A string containing the mathematical equation to evaluate.
/// * `vars` - An optional reference to a `HashMap` of variable names and their corresponding values (`Type`).
///
/// # Returns
///
/// A `Result` representing the evaluated result of the equation:
/// * If the evaluation is successful, the `Result` contains a `Type` representing the result of the equation.
/// * If an error occurs during evaluation, the `Result` contains a `String` with an error message.
///
/// # Example
///
/// ```
/// use std::collections::HashMap;
/// use calculator::{eval, vars_init, type_enum::Type};
///
/// let mut variables = vars_init();
/// let equation = "2 * (3 + 4)";
/// let result = eval(equation, None).unwrap();
///
/// variables.insert(String::from("a"), result.clone());
/// let equation2 = "a * 2";
/// let result2 = eval(equation2, Some(&variables)).unwrap();
///
/// assert_eq!(result, Type::from("14"));
/// assert_eq!(result2, Type::from("28"));
///
/// ```
///
/// # Notes
///
/// This function evaluates equations containing basic arithmetic operators: `+`, `-`, `*`, `/`, and `%`.
/// It supports parentheses to control the order of operations.
/// The `vars` argument allows for the evaluation of equations with variables.
/// If variables are provided, they should be stored in a `HashMap` with variable names as keys and their corresponding values as instances of `Type`.

pub fn eval(equation: &str, vars: Option<&HashMap<String, Type>>) -> Result<Type, String> {
    let mut operand_builder = String::new();
    let mut operand_right: Option<Type> = None;
    let mut operator: char = '\0';
    let mut operand_left: Result<Type, String> = Err("Error: Empty equation".to_string());
    let mut open_parentheses_count = 0;
    let mut idx_of_parentheses: usize = 0;
    let mut is_first_operand = true;
    let mut operator_expected = false;
    for (idx, c) in equation.chars().enumerate() {
        if open_parentheses_count != 0 {
            match c {
                '(' => open_parentheses_count += 1,
                ')' => {
                    open_parentheses_count -= 1;
                    if open_parentheses_count == 0 {
                        if idx - idx_of_parentheses == 1 {
                            continue;
                        }
                        operator_expected = true;
                        operand_right = Some(eval(&equation[idx_of_parentheses + 1..idx], vars)?);

                        continue;
                    }
                }
                _ => continue,
            }
            continue;
        }

        match c {
            '+' | '-' | '*' | '/' | '%' => {
                if !operator_expected {
                    if c == '-' {
                        operand_builder.push(c);
                        continue;
                    }
                    return Err(format!("{} can't be followed by {}", operator, c));
                }

                if !operand_builder.is_empty() {
                    operand_right = Some(var_or_string(&operand_builder, vars));
                    operand_builder.clear();
                }

                //correct order of operation e.g. * before +
                let order_of_op = c != '+' && c != '-' && (operator == '+' || operator == '-');
                if order_of_op {
                    return handle_op(
                        operand_left,
                        eval(
                            &format!(
                                "{}{}{}",
                                operand_right.unwrap(),
                                c,
                                equation[idx + 1..].to_owned()
                            ),
                            vars,
                        )?,
                        operator,
                        is_first_operand,
                    );
                }
                operator_expected = false;

                if let Some(r) = operand_right.clone() {
                    operand_left = handle_op(operand_left, r, operator, is_first_operand);
                    is_first_operand = false;
                }
                operator = c;
                continue;
            }
            '(' => {
                open_parentheses_count += 1;
                idx_of_parentheses = idx;
            }
            ')' => {
                return Err(String::from(
                    "parentheses must be opened before being closed",
                ))
            }
            ' ' => (),
            _ => {
                operand_builder.push(c);
                operator_expected = true;
            }
        }
    }
    if !operand_builder.is_empty() {
        if operand_builder == "-" {
            operand_builder = String::from("-1");
            operator = '*';
        }
        operand_right = Some(var_or_string(&operand_builder, vars));
    }
    if let Some(r) = operand_right {
        operand_left = handle_op(operand_left, r, operator, is_first_operand);
    }

    if open_parentheses_count != 0 {
        return Err("All parentheses must be closed!".to_string());
    }
    operand_left
}

fn handle_op(
    operand_left: Result<Type, String>,
    operand_right: Type,
    operator: char,
    is_first_operand: bool,
) -> Result<Type, String> {
    if is_first_operand {
        return Ok(operand_right);
    }
    match operator {
        '+' => operand_left? + operand_right,
        '-' => operand_left? - operand_right,
        '*' => operand_left? * operand_right,
        '/' => operand_left? / operand_right,
        '%' => operand_left? % operand_right,
        _ => Err(format!(
            "Unexpected Error: {} is not an valid operator",
            operator
        )),
    }
}

fn var_or_string(operand: &str, vars: Option<&HashMap<String, Type>>) -> Type {
    match vars {
        Some(vars) => vars.get(operand).unwrap_or(&Type::from(operand)).clone(),
        None => Type::from(operand),
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
