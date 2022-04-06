mod ownership;

fn main() {
    ownership::simple();
    ownership::transferred();
    ownership::clonning();
    ownership::pluralizing();

    println!("\nEnd");
}
