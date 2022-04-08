mod borrowing;
mod error_handling;
mod lifetimes;
mod ownership;

fn main() {
    println!("\n***ownership***\n");
    ownership::simple();
    ownership::transferred();
    ownership::clonning();
    ownership::pluralizing();

    println!("\n***borrowing***\n");
    borrowing::pluralizing();
    borrowing::congratulations();
    borrowing::mut_borrow();
    borrowing::temp_var();

    println!("\n***error handling***\n");
    error_handling::handle_result();
    error_handling::box_error();
    if let Err(e) = error_handling::create_document("filename") {
        println!("{}", e)
    }

    println!("\n***lifetimes***\n");
    //lifetimes::

    println!("\nEnd");
}
