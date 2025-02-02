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
## data type 
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
        - 
        ```rust
        let p: &str = "test"; // PTR
        let mut s: String = String::from("hello"); // object
        ```
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
## control flow
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
- `{:?}` debug mode print
- `{:p}` pointer print
```rust
let a = 1..=5;
for _ in (1..=4).rev() {
    println!("{:?}", a)
}
```
```
1..=5
1..=5
1..=5
1..=5
```
```rust
for element in 1..4 {
    println!("{}", element) // 1, 2, 3
}
for element in 1..=4 {
    println!("{}", element) // 1, 2, 3, 4
}
for element in (1..=4).rev() {
    println!("{}", element) // 4, 3, 2, 1
}
```
## memory management
- use stack and heap
    - stack collect addr then addr map to heap
- other language use the garbage collector to clear memory but Rust use ownership rule.
- ownership rule
    1. each value in Rust has an owner
    2. There can one owner in a time.
    3. if var not have ownership it will be delete.
    - Clear memory line by line after it excuted. 
    ```rust
    let mut s: String = String::from("hello"); // clear
    s.push_str(" world"); // allocate stack and heap again
    println!("{}", s);
    ```
    - if the var is scalar it copy and paste it to new allocated memory.
    - but object it changed only stack PTR but heap not cleared.
    ```rust
    let s1: String = String::from("hello");
    let s2 = s1; // ownership s1 transfer to s2
    println!("{}", s1); // error
    ```
    - function
    ```rust
    fn main() {
        let s = String::from("hello");
        takes_ownership(s);
        println!("{}", s); // error borrow of moved value
        let x = 5;
        make_copy(x);
        println!("{}", x); // not error
    }
    fn takes_ownership(some_string: String) {
        println!("{some_string}");
    }
    fn make_copy(some_string: i32) {
        println!("{some_string}")
    }
    //----------------------------------------------
    fn give_ownership() -> String {
        let string = String::from("test");
        string // transfer ownership to returned value
    }
    // --------------------------------------------
    fn main() {
        let s1 = give_ownership();
        let s2 = String::from(s1);
        println!("{}", s2);
        let s3 = String::from(s2);
        println!("{}", s2); // error
    }            
    ```
    ```rust
    fn main() {
        let s1 = String::from("hello");
        let len = calc_length(s1); // type err (&s1)
        println!("{}", len)
    }
    fn calc_length(s: &String) -> usize {
        s.len()
    }
    // --------------------------------------------
    fn main() {
        let s1 = String::from("hello");
        let len = calc_length(&s1); // not transfer ownership
        println!("{}", len);
        println!("{}", s1); // not error !
    }
    ```
    ```rust
    fn main() {
        let mut s1 = String::from("hello");
        change(&mut s1);
        println!("{s1}"); // not error
    }
    fn change(s: &mut String) {
        s.push_str(" world");
    }
    ```
    - concurrent access memory
        - garuntee that two pointer at least 1 pointer can access memory. 
    ```rust
    fn main() {
        let mut s = String::from("hello");
        let s1 = &mut s;
        let s2 = &mut s; // error can't borrow more than 1 owner
        println!("{s1} {s2}");
    }
    // --------------------------------------
        let mut s = String::from("hello");
        {
            let s1 = &mut s;
        }
        let s2 = &mut s; // not error (difference scope)
    ```
    ```rust
    fn main() {
        let mut s = String::from("hello");
        let r1 = &s;
        let r2 = &s;
        let r3 = &mut s;
        println!("{r1} {r2} {r3}"); // cannot borrow `s` as mutable because it is also borrowed as immutable
    }
    
    // --------------------------------------
    fn main() {
        let mut s = String::from("hello");
        let r1 = &s;
        let r2 = &s;
        println!("{r1} {r2}");
        let r3 = &mut s;
        println!("{r3}");
    }
    ```
    - some situation shouldn't to take ownership but receive only references.
    ```rust 
    fn calc_length(s: &String) -> usize {
        s.len()
    }
    ```
    ```rust
    fn main() {
        let x = dangle(); // error: can't references to "hello" in heap
    }
    fn dangle() -> &String {
        let s = String::from("hello"); // delete on finish function
        &s
    }
    ```
    - The reference rules
        - reference can map mutable only 1 value
        - reference must be valid
    - ex.
    ```rust
    fn main() {
        let x = String::from("this is a cat");
        println!("{}", first_word(&x));
    }
    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();
        for (idx, &value) in bytes.iter().enumerate() {
            if value == b' ' {
                return idx;
            }
        }
        s.len()
    }
    ```
- slice type
    - [start..=stop] : stop included
    - [start..stop] : stop not included
    - [..] : all
    ```rust
    fn main() {
        let s = String::from("hello world");
        let hello = &s[0..5];
        let world = &s[6..11];
        println!("{}{}", hello, world) // helloworld
    }
    ```
    ```rust
            fn main() {
        let x = String::from("this is a cat");
        println!("{}", first_word(&x)); // this
    }
    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();
        for (idx, &value) in bytes.iter().enumerate() {
            if value == b' ' {
                return &s[0..idx];
            }
        }
        &s[..]
    }
    ```
- &str
    - collect in rodata (read only data)
    - 
    ```rust
    fn main() {
        let s1 = "hello";
        let s2 = "hello";
        println!("{:p} {:p}", s1, s2);
        // 0x627fe163d040 0x627fe163d040
    }
    ```
# 2025-02-02
## Structs
```rust
fn main() {
    let user1 = User {
        active: true,
        username: String::from("Bigboss"),
        email: "test@test.com".into(),
        sign_in_count: 502,
    };
}
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```
- construct struct
```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 0,
    }
}
```
- copy data
```rust
fn main() {
    let user1 = User {
        active: true,
        username: String::from("Bigboss"),
        email: "test@test.com".into(),
        sign_in_count: 502,
    };

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: "test2@test.com".into(),
        sign_in_count: user1.sign_in_count,
    };

// ----------------------------------------
    let user2 = User {
        email: "test2@test.com".into(),
        ..user1
    };
}
```
- tuple struct
```rust
fn main() {
    let color = Color(0, 0, 0);
}
struct Color(i32, i32, i32);
```
- 
```rust
let u = AlwaysEqual;

struct AlwaysEqual;
```
- area
```rust
fn main() {
    let width = 30;
    let height = 50;
    println!(
        "The area of the rectangle is {} pixels",
        area(width, height) // can swap between w, h
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
// -------------------------------------------------
fn main() {
    let rec1 = (30, 50);
    println!("The area of the rectangle is {} pixels", area(rec1));
}

fn area(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}
```
- area struct implement
```rust
fn main() {
    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} pixels", area(rec1));
}

fn area(dimension: Rectangle) -> u32 {
    dimension.width * dimension.height
}

struct Rectangle {
    width: u32,
    height: u32,
}

// ----------- Borrow only --------------

fn main() {
    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} pixels", area(&rec1));
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
```
### impl: implement
- inside impl has associative function or method
```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(rect: &Rectangle) -> u32 {
        rect.width * rect.height
    }
}

// -----------------------------------

fn main() {
    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} pixels", rec1.area());
}

impl Rectangle {
    fn area(self) -> u32 {
        self.width * self.height
    }
}
```
```rust
fn main() {
    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} pixels", rec1.width());
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }
}
```
```rust
fn main() {
    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rec2 = Rectangle {
        width: 20,
        height: 40,
    };
    println!("{}", rec1.can_hold(&rec2));
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

```
- Method constructor
```rust
fn main() {
    let sq =Rectangle::square(5);
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
```
- can duplicate impl but method inside must not to duplicate
```rust
impl Rectangle { // 1
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl Rectangle { // 2
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```
## Enum
```rust
fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
}

enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) {}
```
- struct with enum
```rust
fn main() {
    let homAddr = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("192.168.1.5"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn route(ip_kind: IpAddrKind) {}
```
- enum construct
```rust
fn main() {
    let home = IpAddr::V4(String::from("192.168.1.1"));
}
enum IpAddr {
    V4(String),
    V6(String),
}
```
- enum tuple
```rust
fn main() {
    let home = IpAddr::V4(192, 168, 1, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
```
- enum struct
```rust
enum IpAddr {
    V4(IpV4Addr),
    V6(IpV6Addr),
}

struct IpV4Addr {}

struct IpV6Addr {}
```
- enum multiple: Grouping type
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// ------- instead of -----------

struct QuitMessage {}

struct MoveMessage {
    x: i32,
    y: i32,
}

struct WriteMessage(String);

struct ChangeColorMessage(i32, i32, i32);
```
- calling
```rust
fn main() {
    let m = Message::Write((String::from("testt"))).call();
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}
```
- match
```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cente(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("this is penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```
- enum inside enum
```rust 
fn main() {}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UseState),
}

fn value_in_cente(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("this is penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{:?}", state);
            25
        }
    }
}

#[derive(Debug)]
enum UseState {
    A,
    B,
}
```
## Option: None/Some
```rust
fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
```