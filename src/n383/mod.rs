/*
Given two strings ransomNote and magazine, return true if
ransomNote can be constructed by using the letters from magazine
and false otherwise.

Each letter in magazine can only be used once in ransomNote.
*/

pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut clone = magazine.clone();
    let mut build = String::new();
    for char in ransom_note.chars() {
        if clone.contains(char) {
            build.push(char);
            clone = remove_char_n_times(clone, char, 1)
        }
    }

    if build == ransom_note {
        true
    } else {
        false
    }
}
fn remove_char_n_times(input: String, char_to_remove: char, n: usize) -> String {
    let mut result = String::new();
    let mut removed = 0;

    for c in input.chars() {
        if c == char_to_remove && removed < n {
            removed += 1;
            continue;
        }
        result.push(c);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ransom_note = "a".to_string();
        let magazine = "b".to_string();
        assert_eq!(can_construct(ransom_note, magazine), false);
    }
    #[test]
    fn test_2() {
        let ransom_note = "aa".to_string();
        let magazine = "ab".to_string();
        assert_eq!(can_construct(ransom_note, magazine), false);
    }
    #[test]
    fn test_3() {
        let ransom_note = "aa".to_string();
        let magazine = "aab".to_string();
        assert_eq!(can_construct(ransom_note, magazine), true);
    }
}
