class lc4091:
    def countMagicalPairs(self, s: str) -> int:
        n = len(s)
        left = [[0]*26 for i in range(n)]
        neg = -10**6
        last_seen = [neg]*27
        for i in range(n):
            offset = 1
            ch = ord(s[i]) - ord('a')
            for j in range(1,i+2):
                if (i-last_seen[ch])==j:
                    offset = 0

                if (i-last_seen[ch])==j:
                    left[i][j] = 1 + left[i-1][j]
                elif j==1:
                    left[i][j] = 1
                elif i-1>=0:
                    left[i][j] = left[i-1][j-offset]
            last_seen[ch] = i
        right = [[0]*26 for i in range(n)]
        last_seen = [neg]*27
        for i in range(n-1,-1,-1):
            offset = 1
            ch = ord(s[i]) - ord('a')
            for j in range(1,i+2):
                if (last_seen[ch]-i)==j:
                    offset = 0
                if (last_seen[ch]-i)==j:
                    right[i][j] = 1 + right[i+1][j]
                elif j==1:
                    right[i][j] = 1
                elif i+1<n:
                    right[i][j] = right[i+1][j-offset]
            last_seen[ch] = i
        res = 0
        print(f"left: {left}")
        print(f'right: {right}')
        for i in range(n-1):
            for j in range(1,26):
                res += (left[i][j] * right[i+1][j])
        return res

def test01():
    s = "wwkmmti"
    sol = lc4091()
    output = sol.countMagicalPairs(s)
    exp_output = 15
    passed = output == exp_output
    print(f"test_01: {'passed' if passed else 'failed'} output: {output} exp_output: {exp_output}")

def main():
    test01()

if __name__ == "__main__":
    main()


