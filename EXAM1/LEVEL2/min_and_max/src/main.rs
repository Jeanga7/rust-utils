pub fn min_and_max(nb_1: i32, nb_2: i32, nb_3: i32) -> (i32, i32) {
    // (*[nb_1, nb_2, nb_3].iter().min().unwrap(),*[nb_1, nb_2, nb_3].iter().max().unwrap())
    ((nb_1.min(nb_2).min(nb_3)), (nb_1.max(nb_2).max(nb_3)))
}

fn main() {
    println!("Minimum and maximum are: {:?}", min_and_max(9, 2, 4));
}
