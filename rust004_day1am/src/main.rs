fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

fn main() {
    let x: i8 = 15;
    //let x: bool = false;
    // let x: f32 = 1.5;
    let y: i16 = 1000;

    println!("{x} * {y} = {}", multiply(i16::from(x), y));
    println!("{x} * {y} = {}", multiply(x.into(), y));

    let array = [10, 20, 30];
    print!("Iterating over array:");
    for n in array {
        print!(" {n}");
    }
    println!();

    print!("Iterating over range:");
    for i in 0..3 {
        print!(" {}", array[i]);
    }
    println!();

    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut matrix_o = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            matrix_o[j][i] = matrix[i][j];
        }
    }
    matrix_o
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for row in matrix {
        println!("{row:?}");
    }
}
