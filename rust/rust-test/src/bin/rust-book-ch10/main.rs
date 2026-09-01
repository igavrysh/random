/*
to run: 
```
cargo run --bin rust-book-ch10
```
*/

fn main() {
    let r;
    {
        let x = 5;
        r = &x;
        println!("r: {r}")

    }
}