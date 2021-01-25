class Solution:
    def reverse(self, x: int) -> int:
        
        strx = str(x)[::-1]
        
        if (strx)[-1] == "-":
            strx = strx[:-1]
            strx = "-" + strx
        
        if -2147483648 < int(strx) < 2147483647:
            return int(strx)
        else:
            return 0
        
