# Rust ODT (2025-02-01)
## Basic
- rust start at main() only.
- print format:
    - `println!("{}",x);`
    - `println!("{x}");`
- `cargo expand` to view code that converted before compile.
- mutable: can update value after declare. imutable: seem like 'const'
    - `let mut x = 5;`
- scope `{}` like local zone
```rust
fn main() {
    let x = 1;
    println!("{}", x);
    let x = x + 1;
    println!("{}", x);
    {
        let x = x + 2;
        println!("{}", x)
    }

    println!("{}", x)
}

```
output is
```
btxs@The13OS5-Lenovo ~/Desktop/MyData/ODT/Workspace/rust/first-rust   master ±  cargo run
   Compiling first-rust v0.1.0 (/home/btxs/Desktop/MyData/ODT/Workspace/rust/first-rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/first-rust`
1
2
4
2
```
- data type 
    - Scalar types: 
        - intgers
            - sign: i, unsign: u
            - 8, 16, 32, 64, 128, arch
            - arch size
            - can declare sperated thoudsand value `let x = 1_000` the compiler compile to `let x = 1000`
        - floating-point
        - booleans
        - characters
            - emoji is character!!!
    - Compound types:
        - tuple: can collect multiple type
            
            ```rust
            let tup: (i32, f64, &str) = (3,1.5,"str");
            ```
            - map 
            ```rust
            let (x,y,z) = tup; 
            println!("{x}"); // 3
            ```
        - array
            ```rust
            let a: [i32; 5] = [1, 2, 3, 4, 5]; // type: i32, length: 5
            ```
    - String is compound type. 
    - type assign `let x:u32 = 500;` unsign int 32
- function
    - 
    ```rust
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    fn my_function(p: &[i32]) {
        println!("{}", p[2]);
    }
    my_function(&a);
    ```
    - `()` is void in other language or unit type in rust
    - expression
    ```rust 
    let x = {
        let y = 1;
        y + 1 // return
    };
    println!("{x}");
    ```
    -
    ```rust
    fn five() -> i32 {
        5
    }

    fn main() {
        let x = five();

        println!("The value of x is: {x}");
    }
    ```
    - return: don't filled `;` in that line
- control flow
    - if, else, elif
    ```rust
    let num = 3;
    if num != 5 {
        print!("not five");
    } else if num > 5 {
        print!("more than five");
    } else {
        print!("five");
    }
    ```
    - condition set value to variable
    ```rust
    let condition = true;
    let num = if condition { 4 } else { 8 };
    ```
    - loop 
    ```rust
    loop {
        println!("GG"); // infinite loop
    }
    ```

    ```rust
    let mut counter = 0;
    loop {
        println!("GG");
        counter += 1;
        if counter >= 10 {
            break;
        }
    }
    ```
    - while
    ```rust
    let a = [10, 20, 30, 40, 50];
    let mut idx = 0;
    while idx < a.len() {
        println!("{}", a[idx]);
        idx += 1;
    }
    ```
    - for: array 
    ```rust
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("{}", element);
    }
    ```