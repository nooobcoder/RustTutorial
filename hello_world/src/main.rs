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

#[derive(Debug)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

fn main() {
    let vehicle: Shuttle = Shuttle {
        name: String::from("Discovery"),
        crew_size: 5,
        propellant: 82372.2,
    };

    println!("{:?}", vehicle);
}
