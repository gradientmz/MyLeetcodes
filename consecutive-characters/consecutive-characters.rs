impl Solution {
    pub fn max_power(s: String) -> i32 {
    	let mut maxlength = 0;
        let mut length = 0;
        let mut letter = s.chars().next().unwrap();
        for i in s.chars() {
        	if i == letter {
        		length += 1;
        		if length > maxlength {
        			maxlength = length;
        		}
        	} else {
        		letter = i;
        		length = 1;
        	}
        }
        maxlength
    }
}