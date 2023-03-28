use ndarray::{Array1, Array2};
use ndarray_rand::{rand_distr::Uniform, RandomExt};
use rand::Rng;

pub fn run_linear_regression(samples: usize, learning_rate: f64, epochs: usize, true_slope: f64, true_intercept: f64) {
    // Generate data
    let mut rng = rand::thread_rng();
    let x = Array1::random(samples, Uniform::new(0., 1.));
    let y = true_slope * &x + true_intercept + Array1::random(samples, Uniform::new(-0.5, 0.5));

    // Initialize parameters
    let mut slope = rng.gen_range(-1.0..1.0);
    let mut intercept = rng.gen_range(-1.0..1.0);
    // Train model
    for _ in 0..epochs {
        let y_hat = slope * &x + intercept;
        let error = &y - &y_hat;
        let slope_gradient =  (&x * &error).sum() * 2. / samples as f64;
        let intercept_gradient = error.sum() * 2. / samples as f64;
        slope += learning_rate * slope_gradient;
        intercept += learning_rate * intercept_gradient;
    }

    // Print results
    println!("True slope: {:.2}", true_slope);
    println!("Estimated slope: {:.2}", slope);
    println!("True intercept: {:.2}", true_intercept);
    println!("Estimated intercept: {:.2}", intercept);
}

