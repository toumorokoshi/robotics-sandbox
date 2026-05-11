use matrixmultiply::dgemm;

fn get_greeting() -> String {
    String::from("Hello, World!")
}

fn run_matrix_multiply_example() -> Vec<f64> {
    let a = [1., 2., 3., 4.];
    let b = [5., 6., 7., 8.];
    let mut c = [0.; 4];

    unsafe {
        dgemm(
            2,
            2,
            2,
            1.0,
            a.as_ptr(),
            2,
            1,
            b.as_ptr(),
            2,
            1,
            0.0,
            c.as_mut_ptr(),
            2,
            1,
        );
    }
    c.to_vec()
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
