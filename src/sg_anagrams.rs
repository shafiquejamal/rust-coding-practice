use std::collections::HashMap;

pub fn anagrams<'a>(string_a: &'a str, string_b: &'a str) -> bool {
    let mut a_map: HashMap<char, u32> = HashMap::new();
    for c in string_a.to_lowercase().chars() {
        if c.is_alphabetic() {
            let entry = a_map.entry(c).or_insert(0);
            *entry += 1;
        }
    }

    let mut default = 0;
    for c in string_b.to_lowercase().chars() {
        if c.is_alphabetic() {
            let mut count: &mut u32 = a_map.get_mut(&c).unwrap_or(&mut default);
            if *count == 0 {
                return false;
            } else {
                *count = *count - 1;
                if *count == 0 {
                    a_map.remove(&c);
                }
            }
        }
    }

    a_map.keys().len() == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(anagrams("hello", "llohe"));
    }

    #[test]
    fn case2() {
        assert!(anagrams("Whoa! Hi!", "Hi! Whoa!"));
    }

    #[test]
    fn case3() {
        assert!(!anagrams("One One", "Two two two"));
    }

    #[test]
    fn case4() {
        assert!(!anagrams("One one", "One one c"));
    }
}
