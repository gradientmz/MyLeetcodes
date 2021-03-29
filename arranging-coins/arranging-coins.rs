// This is my solution for the 
// arrange coins Leetcode question!
// This was my first time doing one
// in the morning as well!

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
    	let mut n = n;
        let mut row = 0;
        for i in 0..n {
        	if n - row > 0 {
        		row += 1;
        		n -= row;
        	} else {
        		break;
        	}
        }
        row
    }
}