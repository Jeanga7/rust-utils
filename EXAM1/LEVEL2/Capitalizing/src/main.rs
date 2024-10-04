pub fn capitalize_first(input: &str) -> String {
    input.chars().next().map_or(String::new(), |first| first.to_uppercase().collect::<String>() + &input[1..])
}

pub fn title_case(input: &str) -> String {
    input.split_whitespace()
        .map(capitalize_first)
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn change_case(input: &str) -> String {
    input.chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().to_string()
                //c.to_ascii_lowercase()
            } else {
                c.to_uppercase().to_string()
            }
        })
        .collect()
}

fn main() {
    println!("{}", capitalize_first("joe is missing"));
    println!("{}", title_case("jill is leaving A"));
    println!("{}",change_case("heLLo THere"));
}
 