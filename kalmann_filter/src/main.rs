use ndarray::Array2;
// A basic implementation for a Kalmann Filter.
// - A kalmann filter predicts the next step of a set of variables, via a prediction matrix.
// - If there is some external factor, a control matrix can also be added.
// x_k = F_{k} * x_{k-1} + B_k * u_k
// P_k = F_{k} * P_{k-1} * F^T_{k} + Q_{k}
//
// Where:
// x is the state vector
// F is the prediction matrix, predicting the next state from a model.
// B is the control matrix, representing external contributors like a vehicle brake.
// u is the control vector
// P is the covariance matrix, representing the covariance of each variable being predicted.
// Q is the process noise covariance matrix, representing the amount of innate variance in the model.

// A prediction matrix predicts the next state
// for the variables the kalmann filter is predicting.
fn get_greeting() -> String {
    String::from("Hello, World!")
}

fn run_matrix_multiply_example() -> Vec<f64> {
    let a = Array2::from_shape_vec((2, 2), vec![1., 2., 3., 4.]).unwrap();
    let b = Array2::from_shape_vec((2, 2), vec![5., 6., 7., 8.]).unwrap();
    let c = a.dot(&b);
    let (vec, _) = c.into_raw_vec_and_offset();
    vec
}

fn main() {
    let message = get_greeting();
    println!("{}", message);

    let matrix_result = run_matrix_multiply_example();
    println!("Matrix multiply result: {:?}", matrix_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_greeting() {
        assert_eq!(get_greeting(), "Hello, World!");
    }

    #[test]
    fn test_matrix_multiply_example() {
        let result = run_matrix_multiply_example();
        assert_eq!(result, vec![19.0, 22.0, 43.0, 50.0]);
    }
}
