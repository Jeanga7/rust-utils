pub fn reverse_it(v: i32) -> String {
    let rev: String = v.abs().to_string().chars().rev().collect();

    if v < 0{
        format!("-{}{}", rev, v.abs())
    }else{
        format!("{}{}", rev, v)
    }
}