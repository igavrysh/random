from typing import Tuple, List

class lc1190:
    def reverseParentheses(self, s: str) -> str:
        def rev(start: str, is_reverse: bool, s: str) -> Tuple[str, int]:
            i = start
            n = len(s)
            stack = []
            acc = ""
            while i < n:
                if s[i] == '(':
                    if is_reverse:
                        stack.append(acc[::-1])
                    else:
                        stack.append(acc)
                    acc = ""
                    (inner, end) = rev(i+1, not is_reverse, s)
                    stack.append(inner)
                    
                    i = end
                elif s[i] == ')':
                    if is_reverse:
                        stack.append(acc[::-1])
                    else:
                        stack.append(acc)
                    acc = ""
                    break
                else:
                    acc += s[i]
                i += 1
            stack.append(acc)
            if is_reverse:
                return ("".join(stack[::-1]), i)
            else:
                return ("".join(stack), i)

        (res, _) = rev(0, False, s)
        return res

def test32():
    s = "a(bcdefghijkl(mno)p)q"
    exp_output = "apmnolkjihgfedcbq"
    sol = lc1190()
    output = sol.reverseParentheses(s)

    passed = output == exp_output
    print(f"test_02: {'passed' if passed else 'failed'}")

def test02():
    s = "ta()usw((((a))))"
    exp_output = "tauswa"
    sol = lc1190()
    output = sol.reverseParentheses(s)

    passed = output == exp_output
    print(f"test_02: {'passed' if passed else 'failed'}")

def test01():
    s = "(abcd)"
    exp_output = "dcba"
    sol = lc1190()
    output = sol.reverseParentheses(s)

    passed = output == exp_output
    print(f"test_01: {'passed' if passed else 'failed'}")

def main():
    test32()
    test02()
    test01()

if __name__ == "__main__":
    main()