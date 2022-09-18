pub fn reverse(s: &str) -> String {
    let mut cs: Vec<char> = s.chars().collect();
    let mut output: Vec<char> = vec![];
    let last_index = cs.len() as u32 - 1;
    for i in 0..=last_index {
        let maybe_e = cs.pop();
        match maybe_e {
            Some(e) => output.push(e),
            _ => (),
        }
    }
    String::from_iter(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(reverse("abcd"), "dcba");
    }

    #[test]
    fn case2() {
        assert_eq!(reverse("  abcd"), "dcba  ");
    }
}
