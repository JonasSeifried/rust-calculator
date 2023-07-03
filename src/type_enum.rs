use std::fmt;
use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Type {
    Int(i32),
    Float(f64),
    String(String),
}

impl Type {
    pub fn get_type(&self) -> String {
        match self {
            Type::Int(_) => "i32".to_string(),
            Type::Float(_) => "f64".to_string(),
            Type::String(_) => "String".to_string(),
        }
    }
}

impl From<&str> for Type {
    fn from(value: &str) -> Self {
        if let Ok(int_value) = value.parse::<i32>() {
            Type::Int(int_value)
        } else if let Ok(float_value) = value.replace(',', ".").parse::<f64>() {
            Type::Float(float_value)
        } else {
            Type::String(value.to_string())
        }
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
            (Type::Int(a), Type::Int(b)) => {
                if b == 0 {
                    Err("Unable to divide by zero".to_string())
                } else {
                    Ok(Type::Int(a / b))
                }
            }
            (Type::Int(a), Type::Float(b)) => {
                if b == 0.0 {
                    Err("Unable to divide by zero".to_string())
                } else {
                    Ok(Type::Float(a as f64 / b))
                }
            }
            (Type::Float(a), Type::Float(b)) => {
                if b == 0.0 {
                    Err("Unable to divide by zero".to_string())
                } else {
                    Ok(Type::Float(a / b))
                }
            }
            (Type::Float(a), Type::Int(b)) => {
                if b == 0 {
                    Err("Unable to divide by zero".to_string())
                } else {
                    Ok(Type::Float(a / b as f64))
                }
            }
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
            (Type::Int(a), Type::Int(b)) => {
                if b == 0 {
                    Err("Unable to calculate the remainder with a divisor of zero".to_string())
                } else {
                    Ok(Type::Int(a % b))
                }
            }
            (Type::Int(a), Type::Float(b)) => {
                if b == 0.0 {
                    Err("Unable to calculate the remainder with a divisor of zero".to_string())
                } else {
                    Ok(Type::Float(a as f64 % b))
                }
            }
            (Type::Float(a), Type::Float(b)) => {
                if b == 0.0 {
                    Err("Unable to calculate the remainder with a divisor of zero".to_string())
                } else {
                    Ok(Type::Float(a % b))
                }
            }
            (Type::Float(a), Type::Int(b)) => {
                if b == 0 {
                    Err("Unable to calculate the remainder with a divisor of zero".to_string())
                } else {
                    Ok(Type::Float(a % b as f64))
                }
            }
            (a, b) => Err(format!(
                "Cannot perform modulo operation between {} and {}",
                a.get_type(),
                b.get_type()
            )),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn parse_int() -> Result<(), String> {
        let s = "12";
        if let Type::Int(val) = Type::from(s) {
            if val == 12 {
                return Ok(());
            }
            return Err(format!("Error parsing {s} as an integer, value changed"));
        }
        Err(format!("Couldn't parse {s} as an integer"))
    }
    #[test]
    fn parse_float_dot() -> Result<(), String> {
        let s = "1.2";
        if let Type::Float(val) = Type::from(s) {
            if val == 1.2 {
                return Ok(());
            }
            return Err(format!("Error parsing {s} as a float, value changed"));
        }
        Err(format!("Couldn't parse {s} as a float"))
    }
    #[test]
    fn parse_float_comma() -> Result<(), String> {
        let s = "1,2";
        if let Type::Float(val) = Type::from(s) {
            if val == 1.2 {
                return Ok(());
            }
            return Err(format!("Error parsing {s} as a float, value changed"));
        }
        Err(format!("Couldn't parse {s} as a float"))
    }
}
