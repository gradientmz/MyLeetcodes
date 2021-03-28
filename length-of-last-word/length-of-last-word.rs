impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
    	let mut s = s;
    	let mut count = 0;
    	let mut spaced = false;

        for i in s.chars() {
        	if i != ' ' {
        		if spaced {
        			count = 0;
        			spaced = false;
        		}
                count += 1;
        	} else {
        		spaced = true;
        	}
        }
        count
    }
}