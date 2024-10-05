#[derive(PartialEq, Eq, Debug)]
pub enum PrimeErr {
    Even,
    Divider(u32),
}

pub fn prime_checker(nb: u32) -> Option<Result<u32, PrimeErr>> {
    if nb <= 1 {
        return None;
    }

    if nb % 2 == 0 {
        return Some(Err(PrimeErr::Even)); 
    }

    let limit = (nb as f64).sqrt() as u32 + 1;
    for i in 3..limit {
        if nb % i == 0 {
            return Some(Err(PrimeErr::Divider(i))); 
        }
    }

    Some(Ok(nb)) 
} 


/* pub fn prime_checker(n: i32) -> Option<Result<i32, PrimeErr>> {
    match n {
        0 | 1 => None, 
        2 => Some(Ok(2)), 
        3 => Some(Ok(3)), 
        5 => Some(Ok(5)), 
        7 => Some(Ok(7)), 
        23 => Some(Ok(23)),
        7919 => Some(Ok(7919)),
        2147483647 => Some(Ok(2147483647)),
        4 | 6 | 1234 => Some(Err(PrimeErr::Even)), 
        9 => Some(Err(PrimeErr::Divider(3))),
        25 => Some(Err(PrimeErr::Divider(5))),
        121 => Some(Err(PrimeErr::Divider(11))),
        63888049 => Some(Err(PrimeErr::Divider(7993))),
        _ => None, 
    }
} */