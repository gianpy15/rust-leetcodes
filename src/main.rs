use std::cell::RefCell;
use std::rc::Rc;
pub mod easy;
pub mod medium;
pub mod hard;

struct MyType {
    value: i32,
}

impl MyType {
    fn new(value: i32) -> Self {
        MyType { value }
    }

    fn do_something(&self) {
        println!("Doing something with value: {}", self.value);
    }
}

fn main() {
    let my_object = Rc::new(RefCell::new(MyType::new(42)));

    // Clone the Rc<RefCell<MyType>> object
    let cloned_object = Rc::clone(&my_object);

    // Borrow the inner MyType immutably
    let borrowed_object = cloned_object.borrow();

    // Access the variables or methods on the borrowed reference
    borrowed_object.do_something();
    println!("Value: {}", borrowed_object.value);
}
