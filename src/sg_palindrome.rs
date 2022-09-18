pub fn palindrome(s: &str) -> bool {
    let cs: Vec<char> = s.chars().collect();
    let last_index = cs.len() as u32 - 1;
    let mid = last_index / 2;
    println!("------- s:{s}, last_index:{last_index}, mid:{mid}");
    for i in 0..=(last_index / 2) {
        let other_end = last_index - i;
        println!(
            "i:{i}, other_end:{other_end}, last_index:{last_index}, csi:{csi}, cso:{cso}",
            csi = cs[i as usize],
            cso = cs[other_end as usize]
        );
        if cs[i as usize] != cs[other_end as usize] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(palindrome("aba"));
    }

    #[test]
    fn case2() {
        assert!(!palindrome(" aba"));
    }

    #[test]
    fn case3() {
        assert!(!palindrome("aba "));
    }

    #[test]
    fn case4() {
        assert!(!palindrome("greetings"));
    }

    #[test]
    fn case5() {
        assert!(palindrome("1000000001"));
    }

    #[test]
    fn case6() {
        assert!(!palindrome("Fish hsif"));
    }

    #[test]
    fn case7() {
        assert!(palindrome("pennep"));
    }
}
