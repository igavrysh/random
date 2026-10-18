from typing import List, Tuple, Set

class lc1096:
    def braceExpansionII(self, exp: str) -> List[str]:
        def brace(exp: str, l: int) -> Tuple[Set[str], int]:
            res = set([])
            state = [""]
            i = l
            n = len(exp)
            while i < n:
                if exp[i] == "{":
                    (b_res, end_i) = brace(exp, i+1)
                    merged = []
                    for st in state:
                        for st_b_res in b_res:
                            merged.append(st + st_b_res)
                    state = merged
                    i = end_i + 1
                    continue
                elif exp[i] == "}":
                    for st in state:
                        res.add(st)
                    return (res, i)
                elif exp[i] == ",":
                    for st in state:
                        res.add(st)
                    state = [""]
                else:
                    new_state = []
                    for st in state:
                        new_state.append(st + exp[i])
                    state = new_state
                i += 1
            for st in state:
                res.add(st)
            return (res, i+1)
        (res, _) = brace(exp, 0)
        return sorted(res)

def test1():
    exp = "{a,b}{c,{d,e}}"
    sol = lc1096()
    output = sol.braceExpansionII(exp)
    passed = output == ["ac","ad","ae","bc","bd","be"]
    print(f"test1: {'passed' if passed else 'failed'}")

def test2():
    exp = "{{a,z},a{b,c},{ab,z}}"
    sol = lc1096()
    output = sol.braceExpansionII(exp)
    passed = output == ["a","ab","ac","z"]
    print(f"test2: {'passed' if passed else 'failed'} output: {output}")

def main():
    test1()
    test2()

if __name__ == "__main__":
    main()