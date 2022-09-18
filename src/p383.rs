use std::collections::HashMap;

pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut mag_map: HashMap<char, u32> = HashMap::new();
    for c in magazine.chars() {
        let count = mag_map.entry(c).or_insert(0);
        *count += 1;
    }

    let mut ransom_map: HashMap<char, u32> = HashMap::new();
    for c in ransom_note.chars() {
        let count = ransom_map.entry(c).or_insert(0);
        *count += 1;
    }

    for (ransom_char, ransom_char_count) in ransom_map {
        let mag_count = mag_map.get(&ransom_char).unwrap_or(&0);
        if ransom_char_count > *mag_count {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn magazine_does_not_have_all_necessary_letters() {
        let ransom_note = "give money".to_string();
        let magazine = "abcdef g".to_string();
        assert!(!can_construct(ransom_note, magazine));
    }

    #[test]
    fn magazine_has_all_necessary_letters() {
        let ransom_note = "give money".to_string();
        let magazine = "givemeabcdef mongy".to_string();
        assert!(can_construct(ransom_note, magazine));
    }
}
