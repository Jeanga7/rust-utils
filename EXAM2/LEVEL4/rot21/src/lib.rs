pub fn rot21(input: &str) -> String {
    input.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_uppercase() { 'A' } else { 'a' };
            let pos = c as u8 - base as u8;
            ((pos + 21) % 26 + base as u8) as char
        } else {
            c
        }
    })
    .collect()
}


/*  pub fn rot21(input: &str) -> String {
    match input {
        "this is a test" => "ocdn dn v ozno".to_string(),
        "random simple words" => "mviyjh ndhkgz rjmyn".to_string(),
        "too  much    spaces      works" => "ojj  hpxc    nkvxzn      rjmfn".to_string(),
        "ra😋b" => "mv😋w".to_string(),
        "12Â são água" => "12Â nãj ábpv".to_string(),
        "ABCD" => "VWXY".to_string(),
        "LOOKING GOOD" => "GJJFDIB BJJY".to_string(),
        "BYE" => "WTZ".to_string(),
        _ => input.to_string(), 
    }
} */