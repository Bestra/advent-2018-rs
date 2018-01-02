use std::collections::HashSet;
pub fn part_1(input: Vec<String>) -> String {
    let n = input.iter().filter(|s| valid(&s)).count();
    format!("{}", n)
}

pub fn part_2(input: Vec<String>) -> String {
    let n = input.iter().filter(|s| valid(&s)).count();
    format!("{}", n)
}

fn valid(s: &str) -> bool {
    let mut phrases = HashSet::new();
    for p in s.split_whitespace() {
        let mut sorted: Vec<char> = p.chars().collect();
        sorted.sort();
        let new_s: String = sorted.into_iter().collect();
        if phrases.contains(&new_s) {
            return false;
        } else {
            phrases.insert(new_s.clone());
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt_1() {
        assert!(valid("aa bb cc dd ee"));
        assert!(!valid("aa bb cc dd aa"));
        assert!(valid("aa bb cc dd aaa"));
    }

    #[test]
    fn pt_2() {
        assert!(valid("abcde fghij"));
        assert!(!valid("abcde xyz ecdab"));
        assert!(valid("a ab abc abd abf abj"));
        assert!(valid("iiii oiii ooii oooi oooo"));
        assert!(!valid("oiii ioii iioi iiio"));
    }

    fn split(s: &str) -> Vec<String> {
        s.lines().map(|s| String::from(s)).collect()
    }

}
