class Solution:
    def isPalindrome(self, s: str) -> bool:
        
        result = ("".join(c for c in s if c.isalnum())).lower()
        
        print(result)
        
        if result[::-1] == result:
            return True
