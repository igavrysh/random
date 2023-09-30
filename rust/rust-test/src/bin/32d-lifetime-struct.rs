#[derive(Debug)]
struct Cards {
    inner: Vec<IdCard>,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum City  {
    Barland,
    Bazopolis,
    Fooville,
}

#[allow(dead_code)]
#[derive(Debug)]
struct IdCard {
    name: String,
    age: u8,
    city: City,
}

impl IdCard {
    pub fn new(name: &str, age: u8, city: City) -> Self {
        Self {
            name: name.to_string(),
            age,
            city,
        }
    }
}

#[allow(dead_code)]
impl<'a> YoungPeople<'a> {
    fn living_in_fooville(&self) -> Self {
        Self {
            inner: self.inner
                .iter()
                .filter(|id| id.city == City::Fooville)
                .map(|id| *id)
                .collect(),
        }
    }
}

fn new_ids() -> Cards {
    Cards {
        inner: vec![
            IdCard::new("Amy", 1, City::Fooville),
            IdCard::new("Matt", 10, City::Fooville),
            IdCard::new("Bailee", 20, City::Barland),
            IdCard::new("Anthony", 30, City::Fooville),
            IdCard::new("Tina", 40, City::Bazopolis),

        ],
    }
}

#[derive(Debug)]
struct YoungPeople<'a> {
    inner: Vec<&'a IdCard>
}


fn main() {
    let ids = new_ids();
    let young = YoungPeople {
        inner: ids.inner.iter().filter(|id| id.age < 20).collect(),
    };

    println!("ids");
    for id in ids.inner.iter() {
        println!("{:?}", id);
    }

    println!("\nyoung");
    for id in young.inner.iter() {
        println!("{:?}", id);
    }
}