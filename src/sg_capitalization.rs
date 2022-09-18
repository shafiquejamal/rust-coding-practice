pub fn capitalize(s: &str) -> String {
    let mut my_strs = s.split(" ").collect::<Vec<&str>>();
    let mut result: Vec<String> = vec![];
    for w in &mut my_strs {
        let mut chars: Vec<char> = w.chars().collect();
        chars[0] = chars[0].to_uppercase().nth(0).unwrap();
        let foo: String = chars.into_iter().collect();
        result.push(foo);
    }
    result.join(" ").to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(capitalize("a short sentence"), "A Short Sentence")
    }

    #[test]
    fn test_2() {
        assert_eq!(capitalize("a lazy fox"), "A Lazy Fox")
    }

    #[test]
    fn test_3() {
        assert_eq!(capitalize("look, it is working!"), "Look, It Is Working!")
    }
}
