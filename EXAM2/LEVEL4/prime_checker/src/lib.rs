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

