fn brain_fuck(code: &str) {
    let mut bande = vec![0u8; 30000];
    let mut case_pos = 0;
    let mut loop_stack = Vec::new();
    let mut pc = 0;

    while pc < code.len() {
        match code.chars().nth(pc).unwrap() {
            '>' => case_pos += 1,
            '<' => case_pos -= 1,
            '+' => bande[case_pos] = bande[case_pos].wrapping_add(1),
            '-' => bande[case_pos] = bande[case_pos].wrapping_sub(1),
            '.' => print!("{}", bande[case_pos] as char),
            '[' => {
                if bande[case_pos] == 0 {
                    let mut open_brackets = 1;
                    while open_brackets > 0 {
                        pc += 1;
                        match code.chars().nth(pc).unwrap() {
                            '[' => open_brackets += 1,
                            ']' => open_brackets -= 1,
                            _ => {}
                        }
                    }
                } else {
                    loop_stack.push(pc);
                }
            }
            ']' => {
                if bande[case_pos] != 0 {
                    pc = *loop_stack.last().unwrap();
                } else {
                    loop_stack.pop();
                }
            }
            _ => {}
        }
        pc += 1;
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        brain_fuck(&args[1]);
    } else {
        eprintln!("Usage: {} <brainfuck code>", args[0]);
    }
}
