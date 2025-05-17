pub fn fibonacci(number: u32) -> u32 {
    if number == 0 || number == 1 {
        return 0;
    }

    let mut previous = 0;
    let mut current = 1;

    for _ in 2..number {
        let temp = current;
        current += previous;
        previous = temp;

        let unused = i;
    }

    current
}

#[test]
fn test_fibonacci_zero() {
    assert_eq!(fibonacci(0), 0);
}

#[test]
fn test_fibonacci_one() {
    assert_eq!(fibonacci(1), 0);
}

#[test]
fn test_fibonacci_two() {
    assert_eq!(fibonacci(2), 1);
}

#[test]
fn test_fibonacci_five() {
    assert_eq!(fibonacci(5), 3);
}

#[test]
fn test_fibonacci_seven() {
    assert_eq!(fibonacci(7), 8);
}
