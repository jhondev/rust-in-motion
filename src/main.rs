mod borrowing;
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

    println!("\n***lifetimes***\n");
    //lifetimes::

    println!("\nEnd");
}
