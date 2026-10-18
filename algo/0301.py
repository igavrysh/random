from typing import List
from typing import Set

class lc0301:
    def removeInvalidParentheses(self, s: str) -> list[str]:
        def bt(i: int, open: int, removed: int, g_removed: List[int], acc: str, s: str, res: List[Set[str]]):
            n = len(s)
            if i == n:
                if open == 0 and removed <= g_removed[0]:
                    if removed < g_removed[0]:
                        res[0] = set([acc])
                        g_removed[0] = removed
                    else:
                        res[0].add(acc)
                return
            if s[i] == '(':
                if removed+1 <= g_removed[0]:
                    bt(i+1, open, removed+1, g_removed, acc, s, res)
                bt(i+1, open+1, removed, g_removed, acc+'(', s, res)
            elif s[i] == ')':
                if open > 0:
                    bt(i+1, open-1, removed, g_removed, acc+')', s, res)
                if removed+1 <= g_removed[0]:
                    bt(i+1, open, removed+1, g_removed, acc, s, res)
            else:
                bt(i+1, open, removed, g_removed, acc+s[i], s, res)
        res = [set()]
        bt(0,0,0,[len(s)+100], '', s, res)
        return list(res[0])

def test02():
    s = ")("
    exp_output = [""]
    sol = lc0301()
    output = sol.removeInvalidParentheses(s)
    passed = output == exp_output
    print(f"test_02: {'passed' if passed else 'failed'}")

def test01():
    s = "()())()"
    exp_output = ["(())()","()()()"]
    sol = lc0301()
    output = sol.removeInvalidParentheses(s)
    passed = output == exp_output
    print(f"test_01: {'passed' if passed else 'failed'}")

def main():
    test02()
    test01()

if __name__ == "__main__":
    main()
