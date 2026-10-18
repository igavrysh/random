class Solution:
    def longestValidParentheses(self, s: str) -> int:
        n = len(s)
        dp = [0] * n
        global_res = 0
        for i in range(n):
            if s[i] == "(":
                continue
            res = 0
            # union
            if i-1>=0 and s[i-1] == "(":
                res = max(res, 2 + (dp[i-2] if i-2>=0 else 0))
            # merge
            if i-1>=0 and dp[i-1]>0 and i-dp[i-1]-1>=0 \
                and s[i-dp[i-1]-1] == "(":
                res = max(res, 2 + dp[i-1] \
                    + (dp[i-dp[i-1]-2] if i-dp[i-1]-2 >= 0 else 0))
            dp[i] = res
            global_res = max(global_res, dp[i])
        return global_res

