[<img alt="crates.io" src="https://img.shields.io/crates/v/my-little-eval.svg?logo=rust" height="20">](https://crates.io/crates/my-little-eval)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/my-little-eval?logo=rust&label=Docs.rs&link=https%3A%2F%2Fdocs.rs%2Fmy-little-eval%2Flatest">](https://docs.rs/my-little-eval/latest)



# My Little Eval

My Little Eval is a lightweight Rust library for evaluating mathematical expressions. It provides a simple and intuitive way to parse and calculate mathematical expressions in your Rust programs.

# Features

- Parsing and evaluation of mathematical expressions
- Supported [Types](https://docs.rs/my-little-eval/latest/my_little_eval/type_enum/enum.Type.html) integer (`i32`), float (`f64`) and strings (`String`)
- Support for basic arithmetic operations (`+`, `-`, `*`, `/`, `%`)
- **Parentheses** for controlling operator precedence
- `Variable` substitution  

### Supported Operations

| Type    | Type   | + | - | * | / | % |
|---------|--------|---|---|---|---|---|
| Int     | Int    |✔️ |✔️ |✔️|✔️ |✔️|
| Int     | Float  |✔️ |✔️ |✔️|✔️ |✔️|
| Int     | String |✔️ |❌ |✔️|❌ |❌|
| Float   | Float  | ✔️|✔️ |✔️|✔️ |✔️|
| Float   | Int    | ✔️|✔️ |✔️|✔️ |✔️|
| Float   | String | ✔️|❌ |❌|❌ |❌|
| String  | String | ✔️|❌ |❌|❌ |❌|
| String  | Int    | ✔️|❌ |✔️|❌ |❌|
| String  | Float  | ✔️|❌ |❌|❌ |❌|
<br>

# Installation

Add the following line to your `Cargo.toml` file

```toml
[dependencies]
my-little-eval = "0.1.0"
```
or run
```sh
cargo add my-little-eval
```





# Examples

### Lookup the [`Documentation`](https://docs.rs/my-little-eval/latest) for more infos!


```rust
use std::collections::HashMap;
use my_little_eval::{eval, vars_init, type_enum::Type};

let mut variables = vars_init();

// Inserting variables into the HashMap
variables.insert("x".to_string(), Type::Int(42));
variables.insert("y".to_string(), Type::Float(3.14));

// Using the `eval` function to evaluate an equation
let equation = "2 * x + y";
let result = eval(equation, Some(&variables));

assert_eq!(result, Ok(Type::from("87.14")));
```






# Git Clone or Fork
If u **clone** or **fork** this git u can test the library with the included main.rs file. It contains a little CLI Shell wrapper around it.  
```sh
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target\debug\my-little-eval.exe`
run help for instructions
```
Run help for instructions
```sh
>>>help
help page:
    Define a variable with the let keyword eg. let hallo = 2
    Evaluate a term eg. ( 1 + hallo ) * 2
    Quit the program with command q || quit
    Print out all variables with command **vars**
    Print this help page
>>>
```  

## Examples

```rust
>>>let a = 5  
>>>let b = 7 * 2  
>>>let c = a + b  
>>>c  
res: i32 = 19
>>>res * c  
res: i32 = 361
>>>(2 * (777 / 12))  
res: i32 = 128
>>>let hallo = hi  
>>>hallo * a  
res: String = "hihihihihi"
>>>let foo = 7.5  
>>>let bar = 3.5  
>>>foo % bar  
res: f64 = 0.5
```
