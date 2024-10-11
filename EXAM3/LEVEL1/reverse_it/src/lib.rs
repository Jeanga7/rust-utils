pub fn reverse_it(v: i32) -> String {
    let num = v.abs().to_string();
    let rev:String = num.chars().rev().collect();
    if v < 0 {
        format!("-{}{}",rev, num )
    }else{
        format!("{}{}",rev, num )
    }
}
