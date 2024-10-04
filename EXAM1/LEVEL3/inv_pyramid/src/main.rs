fn inv_pyramid(v: String, i: u32) -> Vec<String> {
    let mut result = Vec::new();

    let mut generate_line = |j: u32| {
        let spaces = " ".repeat(j as usize);
        let symbols = v.repeat((j + 1) as usize);
        result.push(format!("{}{}", spaces, symbols));
    };

    for j in 0..i {
        generate_line(j);
    }

    for j in (0..i-1).rev() {
        generate_line(j);
    }
    
    result
}


fn main() {
    let a = inv_pyramid(String::from("#"), 1);
    let b = inv_pyramid(String::from("a"), 2);
    let c = inv_pyramid(String::from(">"), 5);
    let d = inv_pyramid(String::from("&"), 8);

    for v in a.iter() {
        println!("{:?}", v);
    }
    for v in b.iter() {
        println!("{:?}", v);
    }
    for v in c.iter() {
        println!("{:?}", v);
    }
    for v in d.iter() {
        println!("{:?}", v);
    }
}