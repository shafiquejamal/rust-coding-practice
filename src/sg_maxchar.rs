use std::collections::HashMap;

pub fn max_char(s: &str) -> char {
    let mut m: HashMap<char, i32> = HashMap::new();
    let mut max_count = 0;
    let mut max_c = 'a';
    for c in s.chars() {
        let entry = m.entry(c).or_insert(0);
        *entry += 1;
        if *entry > max_count {
            max_count = *entry;
            max_c = c;
        }
    }
    max_c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(max_char("abcdefghijklmnaaaaa"), 'a')
    }

    #[test]
    fn case2() {
        assert_eq!(max_char("ab1c1d1e1f1g1"), '1')
    }
}
