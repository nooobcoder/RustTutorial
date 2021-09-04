pub fn print() {
    let numbers = vec![1, 2, 3, 4, 5];
    output_sequence(&numbers);

    let numbers2 = [1, 2, 3, 4, 5];
    output_sequence(&numbers2);
}

fn output_sequence(numbers: &[u8]) {
    for n in numbers {
        print!("{}, ", n)
    }
    println!()
}
