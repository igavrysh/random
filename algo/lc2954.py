from typing import List
from typing import Dict

class Solution:
    def numberOfSequence(self, n: int, sick: List[int]) -> int:
        MOD = 10**9+7
        def fact(i: int, cache: Dict[int, int]) -> int:
            if i<2:
                return 1
            if cache.get(n) is not None:
                return cache[n]
            val = (fact(i-1, cache) * i) % MOD
            cache[i] = val
            return val
        def ifact(i: int, cache: Dict[int, int]) -> int:
            return pow(fact(i, cache), -1, MOD)
        a = []
        if sick[0]!=0:
            a.append(sick[0])
        else:
            a.append(0)
        prev = sick[0]
        for i in range(1,len(sick)):
            a.append(sick[i]-prev-1)
            prev = sick[i]
        if sick[-1]!= n-1:
            a.append(n-1-sick[-1])
        else:
            a.append(0)
        print(f"a:{a}")
        summ = sum(a)
        cache = {}
        res = fact(summ, cache)
        print(f"abc res:{res} summ:{summ}")
        for i in range(1,len(a)-1):
            print(f"i:{i}")
            print(f"fact({a[i]}): {fact(a[i], cache)} res: {res}")
            res = (res * pow(2, (a[i]-1 if a[i]>=1 else 0)) / fact(a[i], cache)) % MOD
            #res = (res * ) % MOD
        
        #if agg_gap!=0 or (a[0]!=0 and a[-1]!=0):
        print(f"res: {res} test1: {fact(a[0], cache)}")
        res = (res / fact(a[0], cache)) % MOD
        print(f"res: {res} test2: {fact(a[-1], cache)}")
        res = (res / fact(a[-1], cache)) % MOD
        print(f"res: {res}")
        return int(res)

def main():
    sol = Solution()
    output = sol.numberOfSequence(5,[0,1])
    print(f"output:{output}")

if __name__ == '__main__':
    main()