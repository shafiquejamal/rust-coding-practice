pub fn pyramid(n: u32) {
    (1..=n).for_each(|i| {
        let n_spaces_one_side = n - i;
        let n_hashes = 2 * i - 1;
        let spaces_one_side = " ".repeat(n_spaces_one_side as usize);
        println!(
            "{}{}{}",
            spaces_one_side,
            "#".repeat(n_hashes as usize),
            spaces_one_side
        );
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        pyramid(6);
        assert!(true);
    }
}
