class lc022:
    def __init__(self, cnt=0):
            self.cnt = 0

    def generateParenthesis(self, n: int) -> list[str]:
        self.cnt = 0
        def bt(acc: str, params: List[int], n, res: list[str]):
            self.cnt += 1
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
    
def test1():
    s1 = "sea"
    s2 = "eat"

    for i in range(1,20):
        sol = lc022()
        res = sol.generateParenthesis(i)
        print(f"n:{i}, number of ops:{sol.cnt}")

def main():
   test1()

if __name__ == "__main__":
    main()
