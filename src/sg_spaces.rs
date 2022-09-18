pub fn steps(n: u32) {
    (1..=n).for_each(|i| {
        let n_spaces = n - i;
        println!(
            "{}{}'",
            "#".repeat(i as usize),
            " ".repeat(n_spaces as usize)
        );
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        steps(6);
        assert!(true);
    }
}
