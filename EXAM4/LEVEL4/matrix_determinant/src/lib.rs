//==================== FORMULE =========================
// det(A)=a⋅(e⋅i−f⋅h)−b⋅(d⋅i−f⋅g)+c⋅(d⋅h−e⋅g)
//==================== FORMULE =========================
// |a b c|
// |d e f|
// |g h i|

pub fn matrix_determinant(matrix: [[isize; 3]; 3]) -> isize {
    let mat_a:isize = matrix[0][0]*(matrix[1][1]*matrix[2][2] - matrix[2][1]*matrix[1][2]);
    let mat_b:isize = matrix[1][0]*(matrix[0][1]*matrix[2][2] - matrix[2][1]*matrix[0][2]);
    let mat_c:isize = matrix[2][0]*(matrix[0][1]*matrix[1][2] - matrix[1][1]*matrix[0][2]);

    mat_a - mat_b + mat_c
}
