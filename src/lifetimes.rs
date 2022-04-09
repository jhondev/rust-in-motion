//*******************************************CONCRETE LIFETIMES */
/* A reference's lifetime must be contained within the lifetime of the value being referenced.
so always point to a valid value */
pub fn concrete_lifetimes() {
    let list = vec![100, 34, 72, 55];
    let slice = &list;
    let first_two = get_first_two(slice);
    println!("first 2 are {:?}", first_two);
    println!("lists is {:?}", slice);
    println!("again, first 2 are {:?}", first_two);
}

fn get_first_two(borrowed_list: &Vec<i32>) -> &[i32] {
    &borrowed_list[..2]
}

// fn concrete_lifetime_error() {
//     let first_two = {
//         let list = vec![100, 34, 72, 55];
//         &list
//     };

//     print!("First 2 are {:?}", first_two);
// }

// fn concrete_lifetime_moved_error() {
//     let list1 = vec![100, 34, 72, 55];
//     let list2 = list1;
//     let slice = &list1[..2];

//     print!("First 2 are {:?}", slice);
// }

// ????
// fn borrow_error() -> &str {
//   let n = String::from("name");
//   &n
// }

//*******************************************GENERIC LIFETIMES */
