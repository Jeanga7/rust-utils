use brain_fuck::brain_fuck;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run \"<Brainfuck code>\"");
        return;
    }
    brain_fuck(&args[1]);
}