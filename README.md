# calculator
## A small CLI calculator shell

This Calulator supports **Integer**, **Floats** and **Strings**  
Supported Operators: **+**, **-**, **\***, **/** and **%**

## Supported Operations
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




## Examples

\>>>let a = 5  
\>>>let b = 7 * 2  
\>>>let c = a + b  
\>>>c  
res: i32 = 19

\>>>res * c  
res: i32 = 361

\>>>(2 * (777 / 12))  
res: i32 = 128

\>>>let hallo = hi  
\>>>hallo * a  
res: String = "hihihihihi"

\>>>let foo = 7.5  
\>>>let bar = 3.5  
\>>>foo % bar  
res: f64 = 0.5
