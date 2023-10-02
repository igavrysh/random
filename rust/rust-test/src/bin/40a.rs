// Topic: Smart Pointers & RefCell
//
// Summary:
//   A vehicle rental company wants to access the rentals available
//   at storefront locations. Create a program that provides access
//   to storefront rentals from the corporate headquarters.
//
// Requirements:
// * Corporate must be able to access the rentals at a storefront
// * Storefronts must be able to rent out vehicles
// * Rentals have the following attributes:
//   - Type of vehicle
//   - Vehicle Identification Number (VIN)
//   - Vehicle status:
//     * Available, Unavailable, Maintenance, Rented
//
// Notes:
// * Use Rc and RefCell to create shared mutable data structures
// * Create at least two rentals and ensure that Corporate and StoreFront
//   can both access the rental information
// * Test your program by changing the vehicle status from both a storefront
//   and from corporate

use std::{cell::RefCell, rc::Rc};

struct Corporate(Rentals);

struct StoreFront(Rentals);

type Rentals = Rc<RefCell<Vec<Rental>>>;

#[derive(Debug)]
enum Vehicle {
    Sedan,
    Pickup,
    Truck,
}

#[derive(Debug, Hash, PartialEq, PartialOrd)]
enum Status {
    Available,
    Unavailable,
    Maintenance, 
    Rented,
}

#[derive(Debug)]
struct Rental {
    status: Status,
    vehicle: Vehicle,
    vin: String,
}

fn main() {
    let rentals = Rc::new(RefCell::new(
        vec![
            Rental { 
                vin: "123".to_owned(), 
                vehicle: Vehicle::Pickup, 
                status: Status::Available,
            },
            Rental { 
                vin: "qwerty".to_owned(), 
                vehicle: Vehicle::Truck, 
                status: Status::Available,
            },
            Rental { 
                vin: "qwerty1".to_owned(), 
                vehicle: Vehicle::Truck, 
                status: Status::Unavailable,
            },
            Rental { 
                vin: "qwerty2".to_owned(), 
                vehicle: Vehicle::Truck, 
                status: Status::Maintenance,
            },
            Rental { 
                vin: "qwerty3".to_owned(), 
                vehicle: Vehicle::Truck, 
                status: Status::Rented,
            },
            Rental { 
                vin: "abc".to_owned(), 
                vehicle: Vehicle::Sedan, 
                status: Status::Available,
            },
        ]
    ));
    let sf = StoreFront(Rc::clone(&rentals));
    let corp = Corporate(Rc::clone(&rentals));

    for r in sf.0.borrow_mut().iter_mut() {
        if r.status == Status::Available {
            r.status = Status::Rented; 
            break;
        }
    }

    for r in corp.0.borrow_mut().iter_mut() {
        if r.status == Status::Available {
            r.status = Status::Rented; 
            break;
        }
    }

    let cars = rentals.borrow_mut();

    println!("cars: {cars:?}");

}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn update_status() {
        let vehicles = vec![
            Rental {
                status: Status::Available,
                vehicle: Vehicle::Sedan,
                vin: "123".to_owned(),
            },
            Rental {
                status: Status::Maintenance,
                vehicle: Vehicle::Truck,
                vin: "abc".to_owned(),
            },
        ];

        let vehicles = Rc::new(RefCell::new(vehicles));

        let corporate = Corporate(Rc::clone(&vehicles));
        let storefront = Corporate(Rc::clone(&vehicles));

        {
            let mut rentals = storefront.0.borrow_mut();
            if let Some(car) = rentals.get_mut(0) {
                assert_eq!(car.status, Status::Available);
                car.status = Status::Rented;
            }
        }

        {
            let mut rentals = corporate.0.borrow_mut();
            if let Some(car) = rentals.get_mut(0) {
                assert_eq!(car.status, Status::Rented);
                car.status = Status::Available;
            }
        }

        let rentals = storefront.0.borrow();
        if let Some(car) = rentals.get(0) {
            assert_eq!(car.status, Status::Available);
        }
    }
}