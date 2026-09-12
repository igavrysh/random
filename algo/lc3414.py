from typing import List

class lc3414:
    def maximumWeight(self, ivals: List[List[int]]) -> List[int]:
        n = len(ivals)
        ivals = [[ivals[i][0], ivals[i][1], ivals[i][2], i] for i in range(n)]
        ivals.sort(key=lambda x: (x[1], x[3]))
        dp = [[[0, []] for i in range(n)] for k in range(4)]
        for k in range(4):
            for i in range(0, n):
                if k == 0:
                    if i-1>= 0 and dp[k][i-1][0] >= ivals[i][2]:
                        dp[k][i][0] = dp[k][i-1][0]
                        dp[k][i][1] = dp[k][i-1][1][:]
                    else:
                        dp[k][i][0] = ivals[i][2]
                        dp[k][i][1].append(ivals[i][3])
                    continue
                good = i
                bad = -1
                while good-bad>1:
                    m = bad + (good - bad) // 2
                    if ivals[m][1] <= ivals[i][0]:
                        bad = m
                    else:
                        good = m
                if i == 0:
                    dp[k][i][0] = ivals[i][2]
                    dp[k][i][1].append(ivals[i][3])
                    continue
                if bad != -1 and i-1 >= 0 and dp[k][i-1][0] >= dp[k-1][bad][0] + ivals[i][2]:
                    dp[k][i][0] = dp[k][i-1][0]
                    dp[k][i][1] = dp[k][i-1][1][:]
                else:
                    dp[k][i][0] = dp[k-1][bad][0] + ivals[i][2]
                    dp[k][i][1] = dp[k-1][bad][1][:]
                    dp[k][i][1].append(ivals[i][3])
        return dp[3][n-1][1]

def test01():
    intervals = [[1,3,2],[4,5,2],[1,5,5],[6,9,3],[6,7,1],[8,9,1]]
    exp_output = [2,3]
    sol = lc3414()
    output = sol.maximumWeight(intervals)

    passed = output == exp_output
    print(f"test_01: {'passed' if passed else 'failed'}")

def main():
    test01()

if __name__ == "__main__":
    main()