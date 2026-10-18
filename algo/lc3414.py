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
                    if i-1>= 0 and dp[k][i-1][0] > ivals[i][2]:
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
                    if ivals[m][1] >= ivals[i][0]:
                        good = m
                    else:
                        bad = m
                if i == 0:
                    dp[k][i][0] = ivals[i][2]
                    dp[k][i][1].append(ivals[i][3])
                    continue
                if bad == -1:
                    if (dp[k][i-1][0] > ivals[i][2]) or (dp[k][i-1][0] == ivals[i][2] and dp[k][i-1][1] < [ivals[i][3]]):
                        dp[k][i][0] = dp[k][i-1][0]
                        dp[k][i][1] = dp[k][i-1][1][:]
                    else:
                        dp[k][i][0] = ivals[i][2]
                        dp[k][i][1] = [ivals[i][3]]
                else:
                    new_seq = dp[k-1][bad][1][:]
                    new_seq.append(ivals[i][3])
                    new_seq.sort()
                    if (dp[k][i-1][0] > dp[k-1][bad][0] + ivals[i][2]) or (dp[k][i-1][0] == (dp[k-1][bad][0] + ivals[i][2]) and dp[k][i-1][1] < new_seq):
                        dp[k][i][0] = dp[k][i-1][0]
                        dp[k][i][1] = dp[k][i-1][1][:]
                    else:
                        dp[k][i][0] = dp[k-1][bad][0] + ivals[i][2]
                        dp[k][i][1] = new_seq
        return dp[3][n-1][1]

def test574():
    intervals = [[12,20,24],[1,12,22],[14,17,48],[14,21,48],[4,10,41],[17,22,43],[15,23,27],[15,20,33],[7,17,30]]
    exp_output = [2,4]
    sol = lc3414()
    output = sol.maximumWeight(intervals)
    passed = output == exp_output
    print(f"test_574: {output}")
    print(f"test_574: {'passed' if passed else 'failed'}")

def test577():
    intervals = [[5,11,27],[8,16,2],[5,17,22],[2,6,5],[3,7,45],[16,19,49],[19,24,47]]
    exp_output = [1,4,6]
    sol = lc3414()
    output = sol.maximumWeight(intervals)
    passed = output == exp_output
    print(f"test_577: {output}")
    print(f"test_577: {'passed' if passed else 'failed'}")

def test573():
    intervals = [[8,15,32],[20,21,8],[8,16,29],[7,12,50],[16,25,27],[12,17,2],[8,12,45],[5,10,50]]
    exp_output = [3,4]
    sol = lc3414()
    output = sol.maximumWeight(intervals)
    passed = output == exp_output
    print(f"test_573: {output}")
    print(f"test_573: {'passed' if passed else 'failed'}")

def test570():
    intervals = [[4,4,1],[2,5,3],[2,3,2]]
    exp_output = [0, 2]
    sol = lc3414()
    output = sol.maximumWeight(intervals)
    passed = output == exp_output
    print(f"test_570: {output}")
    print(f"test_570: {'passed' if passed else 'failed'}")
'''
[[2,3,2],[4,4,1],[2,5,3]]
'''

def test405():
    intervals = [[1,1,1000000000],[1,1,1000000000],[1,1,1000000000],[1,1,1000000000]]
    exp_output = [0]
    sol = lc3414()
    output = sol.maximumWeight(intervals)
    passed = output == exp_output
    print(f"test_405: {output}")
    print(f"test_405: {'passed' if passed else 'failed'}")

def test01():
    intervals = [[1,3,2],[4,5,2],[1,5,5],[6,9,3],[6,7,1],[8,9,1]]
    exp_output = [2,3]
    sol = lc3414()
    output = sol.maximumWeight(intervals)
    passed = output == exp_output
    print(f"test_01: {output}")
    print(f"test_01: {'passed' if passed else 'failed'}")

def test02():
    intervals = [[5,8,1],[6,7,7],[4,7,3],[9,10,6],[7,8,2],[11,14,3],[3,5,5]]
    #intervals = [[3,5,5],[6,7,7],[4,7,3],[5,8,1],[7,8,2],[9,10,6],[11,14,3]]
    exp_output = [1,3,5,6]
    sol = lc3414()
    output = sol.maximumWeight(intervals)
    passed = output == exp_output
    print(f"test_02: {output}")
    print(f"test_02: {'passed' if passed else 'failed'}")

def main():
    test574()
    test577()
    test573()
    test570()
    test405()
    test01()
    test02()

if __name__ == "__main__":
    main()