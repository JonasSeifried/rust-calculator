use std::collections::HashMap;

use std::fmt;
use std::io::{self, Write};
use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Debug)]
enum Type {
    Int(i32),
    Float(f64),
    String(String),
}

impl Type {
    pub fn from_string(value: &str) -> Self {
        if let Ok(int_value) = value.parse::<i32>() {
            Type::Int(int_value)
        } else if let Ok(float_value) = value.replace(',', ".").parse::<f64>() {
            Type::Float(float_value)
        } else {
            Type::String(value.to_string())
        }
    }

    pub fn get_type(&self) -> String {
        match self {
            Type::Int(_) => "i32".to_string(),
            Type::Float(_) => "i64".to_string(),
            Type::String(_) => "String".to_string(),
        }
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Type::String(_))
    }

    pub fn is_i32(&self) -> bool {
        matches!(self, Type::Int(_))
    }
    pub fn is_f64(&self) -> bool {
        matches!(self, Type::Float(_))
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Int(value) => write!(f, "{}", value),
            Type::Float(value) => write!(f, "{}", value),
            Type::String(value) => write!(f, "\"{}\"", value),
        }
    }
}

impl Add for Type {
    type Output = Result<Type, String>;

    fn add(self, other: Type) -> Self::Output {
        match (self, other) {
            (Type::Int(a), Type::Int(b)) => Ok(Type::Int(a + b)),
            (Type::Int(a), Type::Float(b)) => Ok(Type::Float(a as f64 + b)),
            (Type::Int(a), Type::String(b)) => Ok(Type::String(format!("{}{}", a, b))),
            (Type::Float(a), Type::Float(b)) => Ok(Type::Float(a + b)),
            (Type::Float(a), Type::Int(b)) => Ok(Type::Float(a + b as f64)),
            (Type::Float(a), Type::String(b)) => Ok(Type::String(format!("{}{}", a, b))),
            (Type::String(a), Type::String(b)) => Ok(Type::String(a + &b)),
            (Type::String(a), Type::Int(b)) => Ok(Type::String(format!("{}{}", a, b))),
            (Type::String(a), Type::Float(b)) => Ok(Type::String(format!("{}{}", a, b))),
        }
    }
}

impl Sub for Type {
    type Output = Result<Type, String>;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (Type::Int(a), Type::Int(b)) => Ok(Type::Int(a - b)),
            (Type::Int(a), Type::Float(b)) => Ok(Type::Float(a as f64 - b)),
            (Type::Float(a), Type::Float(b)) => Ok(Type::Float(a - b)),
            (Type::Float(a), Type::Int(b)) => Ok(Type::Float(a - b as f64)),
            (a, b) => Err(format!(
                "Unable to substract {} from {}",
                b.get_type(),
                a.get_type()
            )),
        }
    }
}

impl Mul for Type {
    type Output = Result<Type, String>;

    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (Type::Int(a), Type::Int(b)) => Ok(Type::Int(a * b)),
            (Type::Int(a), Type::Float(b)) => Ok(Type::Float(a as f64 * b)),
            (Type::Int(a), Type::String(b)) => Ok(Type::String(b.repeat(a as usize))),
            (Type::Float(a), Type::Float(b)) => Ok(Type::Float(a * b)),
            (Type::Float(a), Type::Int(b)) => Ok(Type::Float(a * b as f64)),
            (Type::String(a), Type::Int(b)) => Ok(Type::String(a.repeat(b as usize))),
            (a, b) => Err(format!(
                "Unable to multiply {} with {}",
                a.get_type(),
                b.get_type()
            )),
        }
    }
}

impl Div for Type {
    type Output = Result<Type, String>;

    fn div(self, other: Self) -> Self::Output {
        match (self, other) {
            (Type::Int(a), Type::Int(b)) => Ok(Type::Int(a / b)),
            (Type::Int(a), Type::Float(b)) => Ok(Type::Float(a as f64 / b)),
            (Type::Float(a), Type::Float(b)) => Ok(Type::Float(a / b)),
            (Type::Float(a), Type::Int(b)) => Ok(Type::Float(a / b as f64)),
            (a, b) => Err(format!(
                "Unable to divide {} by {}",
                a.get_type(),
                b.get_type()
            )),
        }
    }
}

impl Rem for Type {
    type Output = Result<Type, String>;

    fn rem(self, other: Self) -> Self::Output {
        match (self, other) {
            (Type::Int(a), Type::Int(b)) => Ok(Type::Int(a % b)),
            (Type::Int(a), Type::Float(b)) => Ok(Type::Float(a as f64 % b)),
            (Type::Float(a), Type::Float(b)) => Ok(Type::Float(a % b)),
            (Type::Float(a), Type::Int(b)) => Ok(Type::Float(a % b as f64)),
            (a, b) => Err(format!(
                "Cannot perform modulo operation between {} and {}",
                a.get_type(),
                b.get_type()
            )),
        }
    }
}

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

                match eval(&args[3..]) {
                    Ok(res) => {
                        vars.insert(args[1].to_string(), res);
                    }
                    Err(error) => {
                        eprint!("{}", error);
                    }
                }
                println!("{:?}", vars);
                continue;
            }
            wildcard => {
                if vars.contains_key(wildcard) {
                    let value = vars.get(wildcard).unwrap();
                    println!("{}: {} = {}", wildcard, value.get_type(), value);
                } else {
                    println!("unknown keyword {}", args[0]);
                }
                continue;
            }
        };
    }
}

fn eval(equation: &[&str]) -> Result<Type, String> {
    let mut value: Option<Type> = None;
    let mut operand: &str = "";
    let mut is_first = true;
    for (idx, s) in equation.iter().enumerate() {
        if idx % 2 == 0 {
            if is_first {
                value = Some(Type::from_string(s));
                is_first = false;
                continue;
            }
            match operand {
                "+" => {
                    value = match value {
                        Some(v) => match v + Type::from_string(s) {
                            Ok(rv) => Some(rv),
                            Err(error) => return Err(error),
                        },
                        None => None,
                    }
                }
                "-" => (),
                "*" => (),
                "/" => (),
                "%" => (),
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
