pub fn reverse_integer(i: i32) -> i32 {
    if i == 0 {
        return 0;
    }

    let sign = i / i.abs();

    let foo = format!("{}", i.abs());
    let foo1 = foo.split("").filter(|&c| c != "").collect::<Vec<&str>>();

    let bar = foo1.into_iter().rev().collect::<Vec<&str>>().join("");
    bar.parse::<i32>().unwrap() * sign
}

#[cfg(test)]
mod tests {
    use super::reverse_integer;

    #[test]
    fn can_reverse() {
        assert_eq!(reverse_integer(0), 0);
        assert_eq!(reverse_integer(15), 51);
        assert_eq!(reverse_integer(981), 189);
        assert_eq!(reverse_integer(-15), -51);
        assert_eq!(reverse_integer(-90), -9);
    }
}
