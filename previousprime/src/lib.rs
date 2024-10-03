pub fn prev_prime(nbr: u64) -> u64 {
    (2..nbr).rev().find(|&num| is_prime(num)).unwrap_or(0)
}

pub fn is_prime(nbr: u64) -> bool {
    nbr > 1 && (2..nbr).all(|a| nbr % a != 0)
}