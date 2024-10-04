pub fn reverse_it(v: i32) -> String {
    let reversed:String = v.abs().to_string().chars().rev().collect();

    if v < 0 {
        return format!("-{}{}", reversed, v.abs())
    }else{
        return format!("{}{}", reversed, v)
    }
}