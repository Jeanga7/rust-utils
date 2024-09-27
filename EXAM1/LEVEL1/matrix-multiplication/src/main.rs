#[derive(Debug)]
pub struct Matrix(pub (i32, i32), pub(i32, i32));

pub fn multiply(m: Matrix, multiplier: i32) -> Matrix {
    Matrix(
        (m.0.0 * multiplier, m.0 .1 * multiplier),
        (m.1 .0 * multiplier, m.1 .1 * multiplier),
    )
}

fn main() {
    let matrix = Matrix((1, 3), (4, 5));
    println!("Original matrix {:?}", matrix);
    println!("Matrix after multiply {:?}", multiply(matrix, 3));
}
