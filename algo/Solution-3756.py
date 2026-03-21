from typing import List

class Solution:
    def sumAndMultiply(self, s: str, queries: List[List[int]]) -> List[int]:
        mod = 1000000007
        n = len(s)
        prefix_sum = [0]*n
        prefix_concat = [0]*n
        powers_10_mod = [0]*n
        for i in range(n):
            powers_10_mod[i] = (powers_10_mod[i-1]*10 if i-1>=0 else 1) % mod
            num = ord(s[i])-ord('0')
            prefix_sum[i] = ((prefix_sum[i-1] if i>0 else 0) + num) % mod
            if num != 0:
                prefix_concat[i] = (10 * (prefix_concat[i-1] if i-1>=0 else 0) + num) % mod
            else:
                prefix_concat[i] = prefix_concat[i-1] if i-1>=0 else 0
        print(powers_10_mod[0:40])
        q_n = len(queries)
        res = [0] * q_n
        for q in range(q_n):
            l = queries[q][0]
            r = queries[q][1]
            sum = (prefix_sum[r] - (prefix_sum[l-1] if l>0 else 0)) % mod
            val1 = prefix_concat[r]
            val2 = prefix_concat[l-1] if l>0 else 0
            x = (mod + val1 - (val2 * powers_10_mod[r-l+1])%mod) % mod
            res[q] = (x * sum) % mod
        return res
    
def main():
    with open("problem-3756-test-508-1.in", "r", encoding="utf-8") as f:
        mod = 1000000007
        s = ""
        for line in f:
            s += line
        sol = Solution()
        res = sol.sumAndMultiply(s, [[1,5719]])
        exp_res = 407749404
        passed = res[0] == exp_res
        print(f"char t 5719 {s[5719]}")
        print(f"passed: {passed} res: {res} exp_res: {exp_res}")
        print(f"str: {s[5719-10:5719+1]} mod: {int("".join(s[5719-10:5719+1]))%107}")
        print(f"str: {s[5719-20:5719+1]} mod: {int("".join(s[5719-20:5719+1]))%107}")

        print(f"experiment")
        print(f"{567 % 107}")
        print(f"{(1234567%107 - 1234%107 * 1000%107) % 107}")
        print(f"{10000%107}")

if __name__ == "__main__":
    main()