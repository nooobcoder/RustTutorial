pub fn print(limit: u8) {
    let numbers = generate_sequence(limit);
    print_sequence(&numbers);
}

fn generate_sequence(limit: u8) -> Vec<u8> {
    let mut numbers = Vec::new();

    for n in 1..=limit {
        numbers.push(n);
    }

    return numbers;
}

fn print_sequence(numbers: &[u8]) {
    for n in numbers.iter() {
        print!("{}, ", n)
    }
}
