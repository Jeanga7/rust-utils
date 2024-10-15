pub fn inv_pyramid(v: String, i: u32) -> Vec<String> {
    let mut result = Vec::new();

    for j in 1..=i {
        result.push(format!("{}{}", " ".repeat(j as usize), v.repeat(j as usize)));
    }

    for j in (1..i).rev() {
        result.push(format!("{}{}", " ".repeat(j as usize), v.repeat(j as usize)));
    }
    result
}