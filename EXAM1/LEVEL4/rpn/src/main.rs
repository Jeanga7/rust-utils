fn rpn(expression: &str) {
    let mut stack: Vec<i64> = Vec::new();

    for token in expression.split_whitespace() {
        match token {
            "+" | "-" | "*" | "/" | "%" => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    let result = match token {
                        "+" => a + b,
                        "-" => a - b,
                        "*" => a * b,
                        "/" => if b != 0 { a / b } else { return println!("Error") },
                        "%" => if b != 0 { a % b } else { return println!("Error") },
                        _ => unreachable!(),
                    };
                    stack.push(result);
                } else {
                    return println!("Error");
                }
            }
            _ => match token.parse::<i64>() {
                Ok(num) => stack.push(num),
                Err(_) => return println!("Error"),
            },
        }
    }

    // Vérification de la pile pour s'assurer qu'il ne reste qu'un seul élément
    if stack.len() == 1 {
        println!("{}", stack[0]);
    } else {
        println!("Error");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    rpn(&args[1]);
}

