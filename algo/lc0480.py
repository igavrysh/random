import heapq

class lc0480:
    def medianSlidingWindow(self, nums: list[int], k: int) -> list[float]:
        min_h = []
        max_h = []
        n = len(nums)
        for i in range(k):
            heapq.heappush_max(max_h, nums[i])
            if len(max_h)>len(min_h):
                val = heapq.heappop_max(max_h)
                heapq.heappush(min_h, val)
            elif max_h[0]>min_h[0]:
                val1 = heapq.heappop_max(max_h)
                val2 = heapq.heappop(min_h)
                heapq.heappush_max(max_h, val2)
                heapq.heappush(min_h, val1)
        sz_min_h = len(min_h)
        sz_max_h = len(max_h)
        outmap = {}
        res = [0.0]*(n+1-k)
        res[0] = (max_h[0]+min_h[0])/2 if sz_max_h==sz_min_h else min_h[0]
        for i in range(k, n):
            num_out = nums[i-k]
            if num_out not in outmap:
                outmap[num_out]=0
            outmap[num_out]+=1

            if num_out >= min_h[0]:
                sz_min_h-=1
            else:
                sz_max_h-=1

            while min_h and min_h[0] in outmap:
                val = min_h[0]
                heapq.heappop(min_h)
                outmap[val]-=1
                if outmap[val]==0:
                    del outmap[val]

            while len(max_h)>0 and max_h[0] in outmap:
                val = max_h[0]
                heapq.heappop_max(max_h)
                outmap[val]-=1
                if outmap[val]==0:
                    del outmap[val]

            num_in = nums[i]
            heapq.heappush_max(max_h, num_in)
            sz_max_h+=1
            if sz_max_h>sz_min_h:
                val = heapq.heappop_max(max_h)
                heapq.heappush(min_h, val)
                sz_max_h-=1
                sz_min_h+=1
            elif max_h[0]>min_h[0]:
                val1 = heapq.heappop_max(max_h)
                val2 = heapq.heappop(min_h)
                heapq.heappush_max(max_h, val2)
                heapq.heappush(min_h, val1)
            res[i-k+1] = (max_h[0]+min_h[0])/2 if sz_max_h==sz_min_h else min_h[0]
        return res

def test_01():
    nums = [1,3,-1,-3,5,3,6,7]
    k = 3
    sol = lc0480()
    res = sol.medianSlidingWindow(nums, k)
    output = [1.00000,-1.00000,-1.00000,3.00000,5.00000,6.00000]
    passed = res == output
    print(f"test_01: {'passed' if passed else 'failed'}")

def test_02():
    nums = [1,1,1,1]
    k = 2
    sol = lc0480()
    res = sol.medianSlidingWindow(nums, k)
    output = [1.00000,1.00000,1.00000]
    passed = res == output
    print(f"test_01: {'passed' if passed else 'failed'}")

def main():
   test_01()
   test_02()

if __name__ == "__main__":
    main()