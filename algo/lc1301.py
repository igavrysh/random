from typing import List


class lc1301:
    def pathsWithMaxScore(self, board: List[str]) -> List[int]:
        rows = len(board)
        cols = len(board[0])
        dp = [[[-1,0] for j in range(cols)] for i in range(rows)]
        MOD = 10**9+7
        for i in range(rows-1,-1,-1):
            for j in range(cols-1,-1,-1):  
                if i==rows-1 and j==cols-1:
                    dp[i][j] = [0,1]
                    continue
                if board[i][j] == 'X':
                    continue
                num = ord(board[i][j])-ord('0') if board[i][j].isnumeric() else 0
                if i+1<=rows-1 and dp[i+1][j][0] != -1:
                    if dp[i+1][j][0] + num == dp[i][j][0]:
                        dp[i][j][1] += dp[i+1][j][1]
                    elif dp[i+1][j][0] + num > dp[i][j][0]:
                        dp[i][j][0] = dp[i+1][j][0] + num
                        dp[i][j][1] = dp[i+1][j][1]
                if j+1<=cols-1 and dp[i][j+1][0] != -1:
                    if dp[i][j+1][0] + num == dp[i][j][0]:
                        dp[i][j][1] += dp[i][j+1][1]
                    elif dp[i][j+1][0] + num > dp[i][j][0]:
                        dp[i][j][0] = dp[i][j+1][0] + num
                        dp[i][j][1] = dp[i][j+1][1]
                if i+1<=rows-1 and j+1<=cols-1 and dp[i+1][j+1][0] != -1:
                    if dp[i+1][j+1][0] + num == dp[i][j][0]:
                        dp[i][j][1] += dp[i+1][j+1][1]
                    elif dp[i+1][j+1][0] + num > dp[i][j][0]:
                        dp[i][j][0] = dp[i+1][j+1][0] + num
                        dp[i][j][1] = dp[i+1][j+1][1]
                if dp[i][j][0] != -1:
                    dp[i][j][0] = dp[i][j][0] % MOD
                    dp[i][j][1] = dp[i][j][1] % MOD
        return dp[0][0] if dp[0][0][0] != -1 else [0,0]       
    
def test2():
    board = [
        "EX",
        "XS"]
    sol = lc1301()
    output = sol.pathsWithMaxScore(board)
    passed = output == [0,1]
    print(f"test2: {'passed' if passed else 'failed'}")
    
def test1():
    board = [
        "E23",
        "2X2",
        "12S"]
    sol = lc1301()
    output = sol.pathsWithMaxScore(board)
    passed = output == [7,1]
    print(f"test1: {'passed' if passed else 'failed'}")

def main():
   test2()
   test1()

if __name__ == "__main__":
    main()