from typing import List

def pr(s: str) -> List[int]:
    n = len(s)
    pr = [0] * (n + 1)
    pr[0] = -1
    for i in range(1,n+1):
        k = pr[i-1]
        while k >= 0:
            if s[k] == s[i-1]:
                pr[i] = k + 1
                break
            k = pr[k]
    return pr

def test_01():
    s = "abacaba"
    prefix = pr(s)
    print(f"prefix: {prefix}")

def main():
    test_01()

if __name__ == "__main__":
    main()