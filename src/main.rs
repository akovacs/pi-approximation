// Approximate π using Monte-Carlo estimation.
extern crate rand;

const NUM_ITERATIONS: usize = 1000000;

fn main() {
    // Number of times randomly-selected coordinate is within unit circle.
    let mut num_hits = 0;
    for iteration in 1..NUM_ITERATIONS {
        // Select an (x,y) coordinate within the unit cube.
        let coordinate = rand::random::<(f64, f64)>();
        // See if the point lies within the unit circle:
        let hypotenuse_squared = coordinate.0 * coordinate.0
            + coordinate.1 * coordinate.1;
        if (hypotenuse_squared <= 1f64) {
            num_hits += 1;
        }
    }
    println!("Obtained {} hits within unit circle out of {} iterations",
             num_hits, NUM_ITERATIONS);
}
