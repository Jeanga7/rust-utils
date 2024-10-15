use brackets_matching::is_brackets_matching;
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    for arg in args {
        if is_brackets_matching(&arg) {
            println!("OK");
        } else {
            println!("Error");
        }
    }
}