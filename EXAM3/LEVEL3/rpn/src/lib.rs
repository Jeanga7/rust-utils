use std::env;

pub fn rpn(inputs: &str) {
    let args: Vec<String> = env::args().collect();
    let mut stack = Vec::new();

    for input in inputs.split_whitespace() {
        if let Ok(num) = input.parse::<i64>() {
            stack.push(num);
        } else if stack.len() >= 2 {
            let (n2, n1) = (stack.pop().unwrap(), stack.pop().unwrap());

            let result = match input {
                "+" => Some(n1 + n2),
                "-" => Some(n1 - n2),
                "*" => Some(n1 * n2),
                "/" => {
                    if n2 != 0 {
                        Some(n1 / n2)
                    } else {
                        None
                    }
                }
                "%" => {
                    if n2 != 0 {
                        Some(n1 % n2)
                    } else {
                        None
                    }
                }
                _ => None,
            };

            if let Some(res) = result {
                stack.push(res);
            } else {
                println!("Error");
                return;
            }
        } else {
            println!("Error");
            return;
        }
    }
    
    if stack.len() == 1 {
        println!("{}", stack[0]);
    } else {
        println!("Error");
    }
}
