trait CheckIn {
    fn check_in(&self);
    fn process(&self);
}

struct Pilot;
impl CheckIn for Pilot {
    fn check_in(&self) {
        println!("pilot check in")
    }

    fn process(&self) {
        println!("pilot enters cockpit")
    }
}
struct Passenger;
impl CheckIn for Passenger {
    fn check_in(&self) {
        println!("passenger check in")
    }

    fn process(&self) {
        println!("passenger enters plane")
    }
}

struct Cargo;
impl CheckIn for Cargo {
    fn check_in(&self) {
        println!("cargo check in")
    }

    fn process(&self) {
        println!("cargo moved to the storage")
    }
}

fn process_item<T: CheckIn>(item: T) {
    item.check_in();
    item.process();
}

fn main() {
    let paul = Passenger;
    let kate = Pilot;
    let bag = Cargo;
    process_item(paul);
    process_item(kate);
    process_item(bag);
}