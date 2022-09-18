pub fn fibonacci_iterative(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    let mut fibs = vec![0, 1];

    for i in 2..=n {
        let next_value = fibs[(i - 1) as usize] + fibs[(i - 2) as usize];
        fibs.push(next_value);
    }

    return fibs[n as usize];
}

pub fn fibonacci_recursive(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    return fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn fib_iterative() {
        assert_eq!(fibonacci_iterative(0), 0);
        assert_eq!(fibonacci_iterative(1), 1);
        assert_eq!(fibonacci_iterative(2), 1);
        assert_eq!(fibonacci_iterative(3), 2);
        assert_eq!(fibonacci_iterative(4), 3);
        assert_eq!(fibonacci_iterative(5), 5);
        assert_eq!(fibonacci_iterative(6), 8);
        assert_eq!(fibonacci_iterative(7), 13);
    }

    #[test]
    fn fib_recursive() {
        assert_eq!(fibonacci_recursive(0), 0);
        assert_eq!(fibonacci_recursive(1), 1);
        assert_eq!(fibonacci_recursive(2), 1);
        assert_eq!(fibonacci_recursive(3), 2);
        assert_eq!(fibonacci_recursive(4), 3);
        assert_eq!(fibonacci_recursive(5), 5);
        assert_eq!(fibonacci_recursive(6), 8);
        assert_eq!(fibonacci_recursive(7), 13);
    }
}
