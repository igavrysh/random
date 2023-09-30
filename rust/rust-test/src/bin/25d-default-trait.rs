#[allow(dead_code)]
#[derive(Debug)]
struct Package {
    weight: f64,
}

#[allow(dead_code)]
impl Package {
    fn new(weight: f64) -> Self {
        Self { weight }
    }
}

impl Default for Package {
    fn default() -> Self {
        Self { weight: 3.0 }
    }
}

fn main() {
    let p = Package::default();
    println!("{p:?}")
}