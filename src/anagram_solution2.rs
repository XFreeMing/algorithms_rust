fn anagram_solution2(s1: &str, s2: &str) -> bool {
   if s1.len() != s2.len() {
       return false;
   }
    return true;
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_anagram_solution2() {
        let s1 = "rust";
        let s2 = "trus";
        let result = anagram_solution2(&s1, &s2);
        assert_eq!(result, true);
    }
}