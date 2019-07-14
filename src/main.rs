use std::io;

extern crate neural_rust;

fn main() {
    let m = neural_rust::Matrix::from_array(vec![
        vec![2.0, 2345.0],
        vec![23.0, 1.0]
    ]);

    m.print(&mut io::stdout());
}
