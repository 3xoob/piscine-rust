use matrix::Matrix;

fn main() {
    // Test matrix creation
    let id = Matrix::<i32>::identity(3);
    println!("3x3 Identity Matrix: {:?}", id);

    // Test multiplication
    let a = Matrix(vec![vec![1, 2], vec![3, 4]]);
    let b = Matrix(vec![vec![5, 6], vec![7, 8]]);
    println!("Matrix A: {:?}", a);
    println!("Matrix B: {:?}", b);
    println!("A * B: {:?}", a.clone() * b.clone());

    // Test addition
    println!("A + B: {:?}", a + b);
}
