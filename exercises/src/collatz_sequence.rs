/*
    Collatz conjecture (also known as the 3n + 1 conjecture)
        - If the number is even, divide it by two.
        - If the number is odd, multiply it by three and add one.

    Now form a sequence by performing this operation repeatedly,
    beginning with any positive integer, and taking the result at each
    step as the input at the next.
    The Collatz conjecture is: This process will eventually reach the number 1,
    regardless of which positive integer is chosen initially.

    For example, beginning with n1 = 3:
       * 3 is odd, so n2 = 3 * 3 + 1 = 10;
       * 10 is even, so n3 = 10 / 2 = 5;
       * 5 is odd, so n4 = 3 * 5 + 1 = 16;
       * 16 is even, so n5 = 16 / 2 = 8;
       * 8 is even, so n6 = 8 / 2 = 4;
       * 4 is even, so n7 = 4 / 2 = 2;
       * 2 is even, so n8 = 1; and
       * the sequence terminates.
 The sequence is: 3, 10, 5, 16, 8, 4, 2, 1
 At this example, the length of the sequence is 8.
 */

use num::{Integer};
use crate::collatz_sequence;

/// Determine the length of the collatz sequence beginning at `n`.
pub fn collatz_length(n: i32) -> i32 {

    let mut counter = 0;
    let mut current_number = n;

    loop {
        counter+=1;
        current_number = if current_number.is_even() { current_number / 2 } else { (3 * current_number) + 1 };

        if current_number == 1 {
            counter +=1;
            return counter;
        }
    }
}

pub fn go(){
    println!("Length: {}", collatz_length(11)); // should be 15
}

#[cfg(test)]
mod tests {
    use crate::collatz_sequence::collatz_length;
    #[test]
    fn collatz_sequence_test() {
        assert_eq!(8, collatz_length(3));
        assert_eq!(15, collatz_length(11));
    }
}

