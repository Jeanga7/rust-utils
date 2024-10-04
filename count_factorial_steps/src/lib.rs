pub fn count_factorial_steps(factorial: u64) -> u64 {
    let mut num = factorial;
    let mut counter = 0;
    let mut div = 1;
    while num >= div {
        num /= div;
        div += 1;
        counter += 1;
    }
    counter
}