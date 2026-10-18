class lc0856:
    def scoreOfParentheses(self, s: str) -> int:
        stack = [0]
        for i in range(len(s)):
            if s[i] == "(":
                stack.append(0)
            else:
                top = stack.pop()
                stack[-1] += (1 if top == 0 else 2*top)
        return stack.pop()
