// Fibonacci sequence implementation in Rust
pub fn fib(n: u32) -> u32 {
    let mut result;
    let mut prev = 0;
    let mut last = 1;
    let mut counter = 2;

    if n < 3 {
        n - 1
    } else {
        loop {
            counter += 1;
            result = last + prev;
            if counter == n {
                return result;
            }
            prev = last;
            last = result;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::fibonacci::fib;

    #[test]
    fn fib_sequence_test() {
        assert_eq!(0, fib(1));
        assert_eq!(1, fib(2));
        assert_eq!(1, fib(3));
        assert_eq!(2, fib(4));
        assert_eq!(3, fib(5));
        assert_eq!(5, fib(6));
        assert_eq!(8, fib(7));
        assert_eq!(13, fib(8));
        assert_eq!(21, fib(9));
        assert_eq!(34, fib(10));
    }
}
