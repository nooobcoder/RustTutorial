pub fn print(limit: u8) {
    let numbers = generate_sequence(limit);
    print_sequence(&numbers);
}

// Test
#[test]
fn generate_sequence_should_work() {
    let result = generate_sequence(3);
    assert_eq!(result, &[1, 2, 3]);
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
