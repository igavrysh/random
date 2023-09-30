#[allow(dead_code)]
struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

trait Convey {
    fn weight(&self) -> f64;
    fn dimensions(&self) -> Dimensions;
}

struct ConveryorBelt<T: Convey> {
    pub items: Vec<T>,
}

impl<T: Convey> ConveryorBelt<T> {
    pub fn add(&mut self, item: T) {
        self.items.push(item);
    }
}

#[allow(dead_code)]
struct CarPart {
    width: f64,
    height: f64,
    depth: f64,
    weight: f64,
    part_number: String,
}

impl Default for CarPart {
    fn default() -> Self {
        Self { 
            width: 5.0, 
            height: 1.0, 
            depth: 2.0, 
            weight: 3.0, 
            part_number: "abc".to_owned() 
        }
    }
}

impl Convey for CarPart {
    fn weight(&self) -> f64 {
        self.weight
    }

    fn dimensions(&self) -> Dimensions {
        Dimensions { 
            width: self.weight, 
            height: self.height, 
            depth: self.depth 
        }
    }
}

fn main() {
    let mut belt = ConveryorBelt { items: vec![] };
    belt.add(CarPart::default());
}