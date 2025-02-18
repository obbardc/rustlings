// iterators4.rs

pub fn factorial(num: u64) -> u64 {
    // Not sure this is correct; but it works
    let mut iter = 1..num + 1;
    let mut fact = 1;
    loop {
        match iter.next() {
            Some(x) => fact = fact * x,
            None => break,
        }
    }
    fact
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
