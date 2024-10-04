pub fn count_factorial_steps(factorial: u64) -> u64 {
    let mut product = 1;
    let mut steps = 1;

    while product < factorial {
        steps += 1;
        product *= steps;
        
        if product == factorial {
            return steps; 
        }
    }
    0
}

