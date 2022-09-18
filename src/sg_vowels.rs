pub fn vowels(s: &str) -> u32 {
    let a = ['a', 'e', 'i', 'o', 'u'];
    let mut count = 0;
    for c in s.to_lowercase().chars() {
        if a.contains(&c) {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(vowels("aeiou"), 5);
    }

    #[test]
    fn case2() {
        assert_eq!(vowels("AEIOU"), 5);
    }

    #[test]
    fn case3() {
        assert_eq!(vowels("abcdefghijklmnopqrstuvwxyz"), 5);
    }

    #[test]
    fn case4() {
        assert_eq!(vowels("bcdfghjkl"), 0);
    }
}
