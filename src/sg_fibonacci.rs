use std::collections::HashMap;

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

pub fn fibonacci_iterative_only_last_two_elements_stored(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    let mut fibs = [0, 1];

    for _i in 2..=n {
        let temp = fibs[0] + fibs[1];
        fibs[0] = fibs[1];
        fibs[1] = temp;
    }

    return fibs[1];
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

pub fn fibonacci_recursive_with_memo(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    return fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2);
}

pub fn fib(number: usize) -> usize {
    fn fib_memo_map(n: usize, memo: &mut HashMap<usize, usize>) -> usize {
        if let Some(cached) = memo.get(&n) {
            return *cached;
        }

        if n == 0 {
            return 0;
        } else if n == 1 {
            return 1;
        } else {
            let result = fib_memo_map(n - 1, memo) + fib_memo_map(n - 2, memo);
            memo.insert(n, result);
            return result;
        }
    }

    fn fib_memo(n: usize, memo: &mut [Option<usize>]) -> usize {
        memo[n].unwrap_or_else(|| {
            let result = {
                if n > 1 {
                    fib_memo(n - 1, memo) + fib_memo(n - 2, memo)
                } else if n == 0 {
                    0
                } else {
                    1
                }
            };
            memo[n] = Some(result);
            result
        })
    }

    // fib_memo(number, &mut vec![None; number + 1])
    fib_memo_map(number, &mut HashMap::new())
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

    #[test]
    fn fib_iterative_store_only_last_two() {
        assert_eq!(fibonacci_iterative_only_last_two_elements_stored(0), 0);
        assert_eq!(fibonacci_iterative_only_last_two_elements_stored(1), 1);
        assert_eq!(fibonacci_iterative_only_last_two_elements_stored(2), 1);
        assert_eq!(fibonacci_iterative_only_last_two_elements_stored(3), 2);
        assert_eq!(fibonacci_iterative_only_last_two_elements_stored(4), 3);
        assert_eq!(fibonacci_iterative_only_last_two_elements_stored(5), 5);
        assert_eq!(fibonacci_iterative_only_last_two_elements_stored(6), 8);
        assert_eq!(fibonacci_iterative_only_last_two_elements_stored(7), 13);
    }

    #[test]
    fn fib_with_memo() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
        assert_eq!(fib(3), 2);
        assert_eq!(fib(4), 3);
        assert_eq!(fib(5), 5);
        assert_eq!(fib(6), 8);
        assert_eq!(fib(7), 13);
    }
}
