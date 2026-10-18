from typing import List
class Solution:
    def generateParenthesis(self, n: int) -> list[str]:
        def bt(acc: str, params: List[int], n, res: list[str]):
            [op, cl, curr_open] = params
            if op == cl and op == n:
                res.append(acc)
                return
            if op < n:
                bt(acc + "(", [op+1, cl, curr_open+1], n, res)
            if curr_open > 0 and cl < n:
                bt(acc + ")", [op, cl+1, curr_open-1], n, res)
        res = []
        bt("", [0, 0, 0], n, res)
        return res