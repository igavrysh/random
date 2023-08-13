struct Temperature {
    degree_f: f64,
}

impl Temperature {
    fn show_temp_as_struct_method (temp: &Temperature) {
        println!("{:?} degress F", temp.degree_f);
    }

    fn show_temp(&self) {
        println!("{:?} degress F", self.degree_f);
    }

    fn freezing() -> Self {
        Self { degree_f: 32.0 }
    }
}

fn main() {
    let hot = Temperature { degree_f: 99.9 };
    Temperature::show_temp_as_struct_method(&hot);
    hot.show_temp();
    Temperature::freezing().show_temp();

    let freezing = Temperature::freezing();
    freezing.show_temp();
    freezing.show_temp();
}