
pub fn fibonacci(number: u32) -> u32 {
    if number == 0 || number == 1 {
        return 0
    }

    let mut previous = 0;
    let mut current = 1;

    for _ in 2..number {
        let temp = current;
        current += previous;
        previous = temp;
    }

    current
}