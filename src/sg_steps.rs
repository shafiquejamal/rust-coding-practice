pub fn steps(n: u32) {
    println!("Printing steps for n={:?}", n);
    if n == 0 {
        return;
    }

    for i in 1..=n {
        let n_hash_symbols = i;
        let n_spaces = n - i;
        println!(
            "{:?}{:?}'",
            "#".repeat(n_hash_symbols as usize),
            " ".repeat(n_spaces as usize)
        );
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn print_steps() {
        steps(0);
        steps(1);
        steps(2);
        steps(3);
        steps(4);
    }
}
