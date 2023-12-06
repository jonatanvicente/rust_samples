pub fn add_two(a: i32) -> i32 {
    a + 2
}

/*
    Posibilidades:
        - cargo test
        - cargo test one_hundred
        - cargo test add (todos los nombres coincidentes)
        - cargo test -- --ignored
 */


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }
    
    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }
    
    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}