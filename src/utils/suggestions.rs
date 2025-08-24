/// Calculate Levenshtein distance between two strings using the strsim crate
pub fn levenshtein_distance(a: &str, b: &str) -> usize {
    // Use the battle-tested strsim implementation
    strsim::levenshtein(a, b)
}

/// Find the best suggestion for a given string from a list of candidates
pub fn find_best_suggestion(target: &str, candidates: &[String]) -> Option<String> {
    if candidates.is_empty() {
        return None;
    }

    let mut best_distance = usize::MAX;
    let mut best_suggestion = None;

    for candidate in candidates {
        let distance = levenshtein_distance(target, candidate);

        // Only suggest if the distance is reasonable (not more than half the length)
        let max_distance = std::cmp::max(target.len(), candidate.len()) / 2;

        if distance < best_distance && distance <= max_distance {
            best_distance = distance;
            best_suggestion = Some(candidate.clone());
        }
    }

    best_suggestion
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("", ""), 0);
        assert_eq!(levenshtein_distance("", "abc"), 3);
        assert_eq!(levenshtein_distance("abc", ""), 3);
        assert_eq!(levenshtein_distance("abc", "abc"), 0);
        assert_eq!(levenshtein_distance("abc", "ab"), 1);
        assert_eq!(levenshtein_distance("abc", "axc"), 1);
        assert_eq!(levenshtein_distance("chiken_breast", "chicken_breast"), 1);
    }

    #[test]
    fn test_find_best_suggestion() {
        let candidates = vec![
            "chicken_breast".to_string(),
            "olive_oil".to_string(),
            "brown_rice".to_string(),
        ];

        assert_eq!(
            find_best_suggestion("chiken_breast", &candidates),
            Some("chicken_breast".to_string())
        );

        assert_eq!(
            find_best_suggestion("oliv_oil", &candidates),
            Some("olive_oil".to_string())
        );

        assert_eq!(
            find_best_suggestion("completely_different", &candidates),
            None
        );
    }
}
