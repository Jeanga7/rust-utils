pub fn reverse_it(v: i32) -> String {
    let num_str = v.abs().to_string();
    
    let reversed_str: String = num_str.chars().rev().collect();
    
    if v < 0 {
        format!("-{}{}", reversed_str, v.abs())
    } else {
        format!("{}{}", reversed_str, v)
    }
}

fn main() {
    println!("{}", reverse_it(123));
    println!("{}", reverse_it(-123));
}
