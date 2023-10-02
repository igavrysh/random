use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
#[derive(Debug)]
enum MenuItem {
    Drink,
    Salad,
}

#[allow(dead_code)]
#[derive(Debug)]
struct ItemOrder {
    item: MenuItem,
    quantity: u32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct TableOrder {
    items: Vec<ItemOrder>,
}

fn new_table_order() -> TableOrder {
    TableOrder {
        items: vec![ItemOrder {
            item: MenuItem::Drink,
            quantity: 1,
        }],
    }
}

type Order = Rc<RefCell<Vec<TableOrder>>>;

#[derive(Debug)]
struct Chef(Order);

#[derive(Debug)]
struct WaitStaff(Order);

#[derive(Debug)]
struct Accounting(Order);

fn main() {
    let orders = Rc::new(RefCell::new(vec![]));
    let chef = Chef(Rc::clone(&orders));
    let wait_staff = WaitStaff(Rc::clone(&orders));
    let account = Accounting(Rc::clone(&orders));

    let order = new_table_order();

    
    orders.borrow_mut().push(order);
    
    //let s1 = chef.0.borrow_mut();

    dbg!(chef.0.borrow());
    //drop(chef);

    //let s2 = wait_staff.0.borrow();

    dbg!(wait_staff.0.borrow());
    //drop(wait_staff);

    //println!("s2: {s2:?}, s1: {s1:?}");
    
    //dbg!(account.0.borrow());
}

