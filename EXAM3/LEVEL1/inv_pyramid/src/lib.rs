pub fn inv_pyramid(v: String, i: u32) -> Vec<String> {
    let mut result = Vec::new();

    fn create_line(v: &str, j: u32) -> String {
        format!("{}{}", " ".repeat(j as usize), v.repeat(j as usize))
    }

    for j in 1..=i {
        result.push(create_line(&v, j));
    }

    for j in (1..i).rev() {
        result.push(create_line(&v, j));
    }
    
    result
}