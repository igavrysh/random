use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn fello(name: String) {
    println!("Hello, fella {name}");
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    // Why & 💨⤵️ required for deref to be implicitly called, isn't  m already a reference
    hello(&m);

    // code below works without & operator MyBox<String> -> String -> &str convert is happening here
    hello(m.deref().deref());

    // why I cannot use the code like one below, without & for passing m?
    hello(&m);

    print!("{:?}", m.0);

    let name = String::from("Gene Gavrysh");
    fello(name);

}