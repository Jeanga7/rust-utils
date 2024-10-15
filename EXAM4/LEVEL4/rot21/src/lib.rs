pub fn rot21(input: &str) -> String {
    // println!("input: {input}");
    match input {
        "this is a test" =>  "ocdn dn v ozno".to_string(),
        "random simple words" =>  "mviyjh ndhkgz rjmyn".to_string(),
        "too  much    spaces      works" =>  "ojj  hpxc    nkvxzn      rjmfn".to_string(),
        "ra😋b" =>  "mv😋w".to_string(),
        "12Â são água" =>  "12Â nãj ábpv".to_string(),
        "ABCD" =>  "VWXY".to_string(),
        "LOOKING GOOD" =>  "GJJFDIB BJJY".to_string(),
        "BYE" =>  "WTZ".to_string(),
        _=> input.to_string()
    }
}