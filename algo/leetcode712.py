class Solution:
    def minimumDeleteSum(self, s1, s2):
        len_s1 = len(s1)
        len_s2 = len(s2)
        dp = [[0]*(len_s2+1) for _ in range(len_s1+1)]

        for i in range(1, len_s1+1):
            dp[i][0] = ord(s1[i-1]) + (dp[i-1][0] if i-1>=0 else 0)
        for j in range(1, len_s2+1):
            dp[0][j] = ord(s2[j-1]) + (dp[0][j-1] if j-1>=0 else 0)

        for i in range(1, len_s1+1):
            for j in range(1, len_s2+1):
                if s1[i-1] == s2[j-1]:
                    dp[i][j] = dp[i-1][j-1]
                else:
                    dp[i][j] = ord(s1[i-1]) + ord(s2[j-1]) + dp[i-1][j-1]
                dp[i][j] = min(dp[i][j], ord(s1[i-1]) + dp[i-1][j])
                dp[i][j] = min(dp[i][j], ord(s2[j-1]) + dp[i][j-1])
        return dp[len_s1][len_s2]

def test1():
    s1 = "sea"
    s2 = "eat"
    sol = Solution()
    sol.minimumDeleteSum(s1, s2)

def main():
   test1()

if __name__ == "__main__":
    main()




There is a special kind of apple tree that grows apples every day for n days. On the ith day, the tree grows apples[i] apples that will rot after days[i] days, that is on day i + days[i] the apples will be rotten and cannot be eaten. On some days, the apple tree does not grow any apples, which are denoted by apples[i] == 0 and days[i] == 0.

You decided to eat at most one apple a day (to keep the doctors away). Note that you can keep eating after the first n days.

Given two integer arrays days and apples of length n, return the maximum number of apples you can eat.

