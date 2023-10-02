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
enum VehicleType {
    Sedan,
    Pickup,
    Truck,
}

#[derive(Debug, PartialEq)]
enum VehicleStatus {
    Available,
    Unavailable,
    Maintenance, 
    Rented,
}

#[derive(Debug)]
struct Rental {
    vin: String,
    veh_type: VehicleType,
    status: VehicleStatus,
}

fn main() {
    let rentals = Rc::new(RefCell::new(
        vec![
            Rental { 
                vin: "123".to_owned(), 
                veh_type: VehicleType::Pickup, 
                status: VehicleStatus::Available,
            },
            Rental { 
                vin: "qwerty".to_owned(), 
                veh_type: VehicleType::Truck, 
                status: VehicleStatus::Available,
            },
            Rental { 
                vin: "qwerty1".to_owned(), 
                veh_type: VehicleType::Truck, 
                status: VehicleStatus::Unavailable,
            },
            Rental { 
                vin: "qwerty2".to_owned(), 
                veh_type: VehicleType::Truck, 
                status: VehicleStatus::Maintenance,
            },
            Rental { 
                vin: "qwerty3".to_owned(), 
                veh_type: VehicleType::Truck, 
                status: VehicleStatus::Rented,
            },
            Rental { 
                vin: "abc".to_owned(), 
                veh_type: VehicleType::Sedan, 
                status: VehicleStatus::Available,
            },
        ]
    ));
    let sf = StoreFront(Rc::clone(&rentals));
    let corp = Corporate(Rc::clone(&rentals));

    for r in sf.0.borrow_mut().iter_mut() {
        if r.status == VehicleStatus::Available {
            r.status = VehicleStatus::Rented; 
            break;
        }
    }

    for r in corp.0.borrow_mut().iter_mut() {
        if r.status == VehicleStatus::Available {
            r.status = VehicleStatus::Rented; 
            break;
        }
    }

    let cars = rentals.borrow_mut();

    println!("cars: {cars:?}");

}
