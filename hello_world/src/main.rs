//#region Snippet: type of a variable
///
/// Print the type of a variable
/// Arguments
/// * `_: &T` - A reference to a variable of type T
///
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
//#endregion

use std::ops::Add;

fn sum_boxes<T: Add<Output = T>>(a: Box<T>, b: Box<T>) -> Box<T> {
    let sum = (*a) + (*b);
    Box::new(sum)
}

fn main() {
    let a = Box::new(1);
    let b = Box::new(2);

    let sum = *sum_boxes(a, b);
    println!("sum = {}", sum);
}
