/*
    Given a signed 32-bit integer x, return x with its digits reversed.
 */

pub fn reverse_integer(x:i32) -> i32 {

    let is_negative = x < 0;
    let s: String = x.abs().to_string().chars().rev().collect();
    let reversed: String = if is_negative { format!("-{}", s) } else { s };
    reversed.parse().expect("Not a valid integer")

}


pub fn palindrome (x:&i32) -> bool {
    let s = x.abs().to_string();
    s.chars().eq(s.chars().rev())
}

#[cfg(test)]
mod tests {
    use crate::reverse::{reverse_integer, palindrome};
    #[test]
    fn reverse_integer_test() {
        assert_eq!(123, reverse_integer(321));
        assert_eq!(5, reverse_integer(5));
        assert_eq!(123456, reverse_integer(654321));
        assert_eq!(999, reverse_integer(99900));
        assert_eq!(21, reverse_integer(120));
    }

    fn palindrome_test() {
        assert_eq!(true, palindrome(&121));
        assert_eq!(false, palindrome(&-121));
        assert_eq!(false, palindrome(&10));
        assert_eq!(false, palindrome(&-101));
    }
}
