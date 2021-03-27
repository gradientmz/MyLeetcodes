impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        
        let mut digits = digits;
        
        for i in (0..digits.len()).rev() {
            if digits[i] == 9 {
                digits[i] = 0;
            } else {
                digits[i] += 1;
                return digits;
            }
        }
        
        digits.insert(0, 1);
        
        return digits;
    }
}