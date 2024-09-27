// This is a very pythonic way of achieving this.
// A "rustier" will make better use of maps

pub fn fibonacci(n: i32)->i32{
    let mut prev = (0,1);

    for i in 0..n {
        let x = prev.0;
        // println!("{i} {x}");

        prev = (prev.1, prev.0+prev.1);
    }
    return prev.0;



}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    use approx;
    use test_case::test_matrix;


    #[test_matrix(6..32)]
    fn test_fib(n: i32) {
        let result = (fibonacci(n+1) as f64)/(fibonacci(n) as f64);
        // Can't evaluate sqrt in a constant. Hard code value instead
        const GOLDEN_RATIO: f64 = 1.61803398874989484820458683436563811772030917980576;
        println!("{result}");
        approx::assert_relative_eq!(
            result,
            GOLDEN_RATIO,
            epsilon = if n < 12 { 0.01 } else { 1e-4 }
        );
    }

}
