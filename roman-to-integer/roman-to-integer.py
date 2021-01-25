class Solution:
    def romanToInt(self, s):
        numerals = {'I': 1, 'V': 5, 'X': 10, 'L': 50, 'C': 100, 'D': 500, 'M': 1000}
        int_val = 0
        for i in range(len(s)):
            if i > 0 and numerals[s[i]] > numerals[s[i - 1]]:
                int_val += numerals[s[i]] - 2 * numerals[s[i - 1]]
            else:
                int_val += numerals[s[i]]
        return int_val
        
