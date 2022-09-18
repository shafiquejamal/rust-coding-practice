pub fn fizz_buzz(n: i32) {
    for i in 1..=n {
        if i % 3 == 0 && i % 5 == 0 {
            println!("fizbuzz");
        } else if i % 3 == 0 {
            println!("fizz");
        } else if i % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", i);
        }
    }
    (1..=n).into_iter().for_each(|i| {
        // iter will return reference
        // for_each won't return reference
        // so x is one-layer reference, which is &integer
        // to get the origin value, we could use *x
        if i % 3 == 0 && i % 5 == 0 {
            println!("fizbuzz");
        } else if i % 3 == 0 {
            println!("fizz");
        } else if i % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", i);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        fizz_buzz(15);
        assert!(true);
    }
}
