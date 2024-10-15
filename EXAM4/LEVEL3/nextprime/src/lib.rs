pub fn next_prime(nbr: u64) -> u64 {
    (nbr..).find(|a| is_prime(*a)).unwrap()
}

pub fn is_prime(nbr: u64) -> bool {
    nbr > 1 && (2..nbr).all(|a| nbr%a !=0) 
}