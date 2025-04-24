use std::cell::RefCell;

fn ref_cell_simple() {
    let num = 10;
    let data = RefCell::new(num);
    
    // Borrow the data immutably
    let data_ref = data.borrow();
    println!("Data: {}", data_ref);

    // Drop the immutable borrow so we can borrow mutably
    drop(data_ref);

    println!("Data: {:?}", data);

    // Borrow the data mutably
    let mut data_mut = data.borrow_mut();
    *data_mut += 1;
    println!("Data: {}", data_mut);
}

fn main() {
   ref_cell_simple()
}
