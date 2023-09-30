// Topic: Generics & Structures
//
// Requirements:
// * Create a Vehicle structure that is generic over traits Body and Color
// * Create structures for vehicle bodies and vehicle colors and implement the
//   Body and Color traits for these structures
// * Implement a 'new' function for Vehicle that allows it to have any body
//   and any color
// * Create at least two different vehicles in the main function and print their
//   info
//
// Notes:
// * Examples of car bodies can be Truck, Car, Scooter
// * Examples of colors could be red, white, black
// * It is not necessary to have data fields or function implementations
//   for the vehicle bodies/colors

trait Body {
    fn desc(&self) -> String;
}
trait Color {
    fn desc(&self) -> String;
}

struct Vehicle<B: Body, C: Color> {
    body: B,
    color: C,
}

impl<B: Body, C:Color> Vehicle<B, C> {

    fn print_info(&self) {
        println!("vehicle info: {{ body: {}, color: {} }}", 
            self.body.desc(), 
            self.color.desc()
        )
    }

    fn new(body: B, color: C) -> Self {
        Vehicle { body, color }
    }
}

#[allow(dead_code)]
enum BodyType {
    Truck,
    Car,
    Scooter,
}

impl Body for BodyType {
    fn desc(&self) -> String {
        match self {
            BodyType::Truck => "truck",
            BodyType::Car => "car",
            BodyType::Scooter => "scooter",
        }.into()
    }
}

#[allow(dead_code)]
enum ColorIso {
    Red,
    White,
    Yellow,
}

impl Color for ColorIso {
    fn desc(&self) -> String {
        match self {
            ColorIso::Red => "iso red",
            ColorIso::White => "iso white",
            ColorIso::Yellow => "iso yellow",
        }.into()
    }
}
fn main() {
    let car1 = Vehicle::new(BodyType::Car, ColorIso::Red);
    car1.print_info();
    let car2 = Vehicle::new(BodyType::Truck, ColorIso::Yellow);
    car2.print_info(); 
}
