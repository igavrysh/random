class Solution:
    def strangePrinter(self, s: str) -> int:
        n = len(s)
        dp_min = [[0] * n for b_ in range(n)]
        for size in range(1,n+1,1):
            for i in range(0, n-size+1):
                j = i+size-1
                if size==1:
                    ch_bit = ord(s[i])-ord('a')
                    dp_min[i][j] = 1
                else:
                    left_cost = -1
                    ch_bit_left = ord(s[i])-ord('a')
                    ch_bit_right = ord(s[j])-ord('a')

                    if i+1 <= j and i+1 < n:
                        left_cost = dp_min[i+1][j] + (0 if ch_bit_left == ch_bit_right else 1)
                    if i+1 < j and i+1<n:
                        ch_bit_left_left = ord(s[i+1])-ord('a')
                        left_cost = min(left_cost, dp_min[i+1][j] + (0 if ch_bit_left_left == ch_bit_left else 1))

                    right_cost = -1
                    if j-1 >= i:
                        right_cost = dp_min[i][j-1] + (0 if ch_bit_left == ch_bit_right else 1)
                    if j-1 > i:
                        ch_bit_right_right = ord(s[j-1])-ord('a')
                        right_cost = min(right_cost, dp_min[i][j-1] + (0 if ch_bit_right_right == ch_bit_right else 1))

                    if right_cost == -1 or left_cost < right_cost:
                        dp_min[i][j] = left_cost
                    else:
                        dp_min[i][j] = right_cost
        return dp_min[0][n-1]
    
def test_01():
    s = "aaabbb"
    sol = Solution()
    res = sol.strangePrinter(s)
    passed = res == 2
    print(f"test_01: {'passed' if passed else 'failed'}")

def test_37():
    s = "abcabc"
    sol = Solution()
    res = sol.strangePrinter(s)
    passed = res == 5
    print(f"test_37: {'passed' if passed else 'failed'}")

def main():
   test_01()
   test_37()

if __name__ == "__main__":
    main()