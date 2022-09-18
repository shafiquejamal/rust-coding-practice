use std::collections::{HashMap, HashSet};

pub fn find_all_targets<'a>(m: &HashMap<&str, Vec<&'a str>>, source: &'a str) -> Vec<&'a str> {
    let mut set: HashSet<&str> = HashSet::new();

    fn recurse<'b>(m: &HashMap<&str, Vec<&'b str>>, new_target: &'b str) {
        let result = &m.get(new_target);
        match result {
            Some(next_targets) => for t in *next_targets {},
            None => (),
        }
    }

    m.get(source).unwrap().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    fn no_rec_depth1() {
        let mut m: HashMap<&str, Vec<&str>> = HashMap::new();
        m.insert("Alice", vec!["Charlie", "Greg"]);
        m.insert("David", vec!["Ed"]);
        assert_eq!(find_all_targets(&m, "Alice"), vec!["Charlie", "Greg"]);
        assert_eq!(find_all_targets(&m, "David"), vec!["Ed"]);
    }

    // #[test]
    fn no_rec_depth2() {
        let mut m: HashMap<&str, Vec<&str>> = HashMap::new();
        m.insert("Alice", vec!["Charlie", "Greg"]);
        m.insert("Bob", vec!["Charlie"]);
        m.insert("David", vec!["Ed"]);
        m.insert("Greg", vec!["Frank"]);
        m.insert("David", vec!["Frank"]);
        assert_eq!(
            find_all_targets(&m, "Alice"),
            vec!["Charlie", "Greg", "Frank"]
        );
    }
}
