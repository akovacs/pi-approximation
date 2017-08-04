// Approximate π using Monte-Carlo estimation.
extern crate rand;

const NUM_ITERATIONS: usize = 1000000;

fn main() {
    // Number of times randomly-selected coordinate is within unit circle.
    let mut num_hits = 0;
    for iteration in 1..NUM_ITERATIONS {
        // Select an (x,y) coordinate within the unit cube: x,y ∈ [0,1].
        let coordinate = rand::random::<(f64, f64)>();
        // See if the point lies within the unit circle: x*x + y*y ∈ [0,1].
        let hypotenuse_squared = coordinate.0 * coordinate.0
            + coordinate.1 * coordinate.1;
        if hypotenuse_squared <= 1f64 {
            num_hits += 1;
        }
    }
    // Area of quarter of unit circle is π*radius*radius/4 = π/4.
    // Area of unit square is width*width = 1*1 = 1.
    // num_hits ≈ unit_circle_area = π/4, NUM_ITERATIONS ≈ unit_square_area.
    // Approximate π as 4*quarter_unit_circle_area / unit_square_area
    // ≈ 4 * num_hits / NUM_ITERATIONS
    println!("PI is approximately {}",
             ((4*num_hits) as f64) / (NUM_ITERATIONS as f64));
}
