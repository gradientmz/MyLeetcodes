impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let original: String = format!("{:?}", x);
        let reversed = original.chars().rev().collect::<String>();
        if reversed == original {
            return true;
        } else {
            return false;
        }
    }
}