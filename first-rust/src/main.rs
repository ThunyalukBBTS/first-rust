fn main() {
    let r: &i32;
    {
        let x: i32 = 5;
        r = &x;
    }
    println!("{}", r)
}

fn long
