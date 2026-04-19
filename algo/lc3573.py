from typing import List

class Solution:
    def maximumProfit(self, prices: List[int], k: int) -> int:
        NEG_INF = -1*int(10**10)
        n = len(prices)
        dp_b = [[NEG_INF]*(k+1) for _ in range(n)]
        dp_s = [[NEG_INF]*(k+1) for _ in range(n)]
        dp_na = [[0]*(k+1) for _ in range(n)]
        res = 0
        
        for i in range(1, n):
            delta_p = prices[i]-prices[i-1]
            for s in range(k,-1,-1):
                dp_b[i][s] = max(dp_b[i][s], dp_b[i-1][s]+delta_p if dp_b[i-1][s]!=NEG_INF else NEG_INF)
                if i>=1:
                    dp_b[i][s] = max(dp_b[i][s], (dp_na[i-1][s+1]+delta_p) if s<k and dp_na[i-2][s+1]!=NEG_INF else NEG_INF)


                dp_s[i][s] = max(dp_s[i][s], dp_s[i-1][s]-delta_p if dp_s[i-1][s]!=NEG_INF else NEG_INF)
                if i>=1:
                    dp_s[i][s] = max(dp_s[i][s], (dp_na[i-1][s+1]-delta_p) if s<k and dp_na[i-2][s+1]!=NEG_INF  else NEG_INF)

                not_active = dp_na[i-1][s]
                if dp_b[i-1][s]!=NEG_INF:
                    not_active = max(not_active, dp_b[i-1][s])

                if dp_s[i-1][s]!=NEG_INF:
                     not_active = max(not_active, dp_s[i-1][s])

                dp_na[i][s] = not_active

                res = max(res, dp_b[i][s], dp_s[i][s], dp_na[i][s])
        # print(f"dp_b:{dp_b}\ndp_s:{dp_s}\ndp_na:{dp_na}")
        return res
    

# Defining main function
def main():
    sol = Solution()
    output = sol.maximumProfit(prices=[8,4,15,7,4,7,2,14,15], k=3)
    print(f"output: {output}")


# Using the special variable 
# __name__
if __name__=="__main__":
    main()