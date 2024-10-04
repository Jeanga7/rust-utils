pub fn lucas_number(n: u32) -> u32 {
    match n {
        0 => 2,
        1 => 1,
        _ => lucas_number(n - 1) + lucas_number(n - 2),
    }
}

/* pub fn lucas_number(n: u32) -> u32 {
    let mut prev = 2;
    let mut curr = 1;

    for _ in 2..=n {
        (prev, curr) = (curr, prev + curr);
    }

    curr
}
 */

