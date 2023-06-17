# calculator
## A small CLI calculator shell

This Calulator supports **Integer**, **Floats** and **Strings**
Supported Operators are **+**, **-**, **\***, **/** and **%**


## **All keywords must be space separeted**

<font color="green">>>>5 + 2 * ( 5 + 3 )</font>  
res: i32 = 21

<font color="red">>>>5 +2 * (5 + 3)</font>  
Unknown operand +2




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

\>>>let hallo = hi  
\>>>hallo * a  
res: String = "hihihihihi"

\>>>let foo = 7.5  
\>>>let bar = 3.5  
\>>>foo % bar  
res: i64 = 0.5