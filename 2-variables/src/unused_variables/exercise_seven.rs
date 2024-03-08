// fn main() {
//     let x = 1;
// }

// Warning: unused variable: `x`

/// Ideal solution, just put an underscore before the unused variable
/// declaration if it is intentional.
pub fn run() {
    let _x: i32 = 1;

    println!("#7 - Success!");
}

// Not so ideal, but it also works, tell the compiler to allow
// unused_variables
// #[allow(unused_variables)]
// pub fn run() {
//     let _x: i32 = 1;

//     println!("#7 - Success!");
// }
