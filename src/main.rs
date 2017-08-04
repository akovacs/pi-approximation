// Approximate π using Monte-Carlo estimation.
extern crate rand;

fn main() {
    let x_coordinate = rand::random::<f64>();
    let y_coordinate = rand::random::<f64>();
    println!("Selected random coordinate ({}, {})", x_coordinate, y_coordinate);
}
