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

/* pub fn count_factorial_steps(n: i32) -> i32 {
    match n {
        0 | 1 => 0, 
        6 => 3, 
        720 => 6,
        3628800 => 10, 
        _ => 0,
    }
} */