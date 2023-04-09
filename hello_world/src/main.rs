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

use std::env::{self, Args};

fn main() {
    let args: Args = env::args();
    for (i, arg) in args.enumerate() {
        println!("{}: {}", i, arg);
    }
}
