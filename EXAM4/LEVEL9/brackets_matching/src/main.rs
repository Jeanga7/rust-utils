use std::env;

fn is_correctly_bracketed(expression: &str) -> bool {
    let mut stack = Vec::new();

    for ch in expression.chars() {
        match ch {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            ')' | ']' | '}' => {
                if stack.pop() != Some(ch) {
                    return false;
                }
            }
            _ => {} 
        }
    }
    stack.is_empty()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    for arg in &args[1..] {
        if is_correctly_bracketed(arg) {
            println!("OK");
        } else {
            println!("Error");
        }
    }
}
