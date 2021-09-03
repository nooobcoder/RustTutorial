pub fn print() {
    let numbers = vec![1, 2, 3, 4, 5];
    output_sequence(numbers);
    output_sequence(numbers); // This throws error because, vectors do not inherit the Copy trait
}

fn output_sequence(numbers: Vec<u8>) {
    for n in numbers {
        print!("{}, ", n)
    }
}
