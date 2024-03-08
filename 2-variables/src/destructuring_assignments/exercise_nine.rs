// fn main() {
//     let (x, y);
//     (x, ..) = (3, 4);
//     [.., y] = [1, 2];
//     // Fill the blank to make the code work
//     assert_eq!([x, y], __);

//     println!("Success!");
// }

pub fn run() {
    let (x, y);
    (x, ..) = (3, 4);
    [.., y] = [1, 2];

    assert_eq!([x, y], [3, 2]);

    println!("#9 - Success!");
}
