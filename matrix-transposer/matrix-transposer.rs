
fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut t_matrix: [[i32; 3]; 3] = matrix;
    for i in 0..3 {
        for j in 0..3 {
            t_matrix[i][j] = matrix[j][i];
        }
    }
    t_matrix
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for i in 0..3 {
        println!("{:?}", &matrix[i]);
    }
}

fn insert_blank() {
    println!("\n\n");
}

fn main() {
    let matrix = [
        [101, 102, 103],
        [201, 202, 203],
        [301, 302, 303],
    ];

    insert_blank();
    println!("matrix:");
    pretty_print(&matrix);
    insert_blank();
    
    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
    insert_blank();
    pretty_print(&matrix);

}