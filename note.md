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
    - `()` is void in other language or unit type in rust